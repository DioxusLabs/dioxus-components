function focusable(element: HTMLElement) {
  if (element.tabIndex < 0 || element.getAttribute("disabled")) {
    return false;
  }

  switch (element.tagName) {
    case "A":
      return (
        !!(element as HTMLAnchorElement).href &&
        (element as HTMLAnchorElement).rel !== "ignore"
      );
    case "INPUT":
      return (element as HTMLInputElement).type !== "hidden";
    case "BUTTON":
    case "SELECT":
    case "TEXTAREA":
      return true;
    default:
      return false;
  }
}

// Records which dialogs made an element inert. The value is a space separated list of
// owner ids so that stacked dialogs compose: closing the top one only removes its own
// id and the element stays inert while a dialog underneath still needs it.
//
// An element the application marked inert before the dialog opened carries no marker, so it
// is left alone.
const INERT_OWNER_ATTR = "data-inert-by";

// The containers of the traps that are currently marking the background, keyed by owner
// id, in the order they were installed. The whole inert state is recomputed from this
// map, so opening and closing a dialog on top of another one composes by construction.
const inertOwners = new Map<string, HTMLElement>();

function readOwners(element: HTMLElement): string[] {
  return (element.getAttribute(INERT_OWNER_ATTR) ?? "").split(" ").filter(Boolean);
}

function inertBackgroundFor(owner: string, container: HTMLElement) {
  inertOwners.set(owner, container);
  refreshInert();
}

function releaseInert(owner: string) {
  inertOwners.delete(owner);
  refreshInert();
}

// Recompute which elements are inert from the dialogs that are currently open. Clearing
// first and marking again keeps the two directions in one place: a dialog that opens
// inside a subtree an earlier dialog marked has to make that subtree reachable again,
// which no amount of marking alone can do.
function refreshInert() {
  for (const element of Array.from(
    document.querySelectorAll<HTMLElement>(`[${INERT_OWNER_ATTR}]`)
  )) {
    element.removeAttribute(INERT_OWNER_ATTR);
    element.removeAttribute("inert");
  }

  const open = Array.from(inertOwners).filter(([, container]) =>
    container.isConnected
  );
  if (open.length === 0) {
    return;
  }

  // The dialog installed last is the one on top, so nothing on its path to `<body>` may
  // be inert — that path contains the dialog itself. Every other open dialog still marks
  // its own background, which is how the one on top covers the ones underneath.
  const top = open[open.length - 1][1];
  const onTopPath = new Set<Element>();
  for (let node: Element | null = top; node; node = node.parentElement) {
    onTopPath.add(node);
  }

  for (const [owner, container] of open) {
    markBackgroundInert(container, owner, onTopPath);
  }
}

// Mark everything outside of `container` inert, so background content is removed from the
// accessibility tree and cannot be reached by a pointer or a programmatic `focus()`.
// `aria-modal` alone covers none of that reliably.
function markBackgroundInert(
  container: HTMLElement,
  owner: string,
  onTopPath: Set<Element>
) {
  let node: HTMLElement = container;
  while (node !== document.body && node.parentElement) {
    const parent = node.parentElement;
    for (const sibling of Array.from(parent.children)) {
      if (
        sibling !== node &&
        sibling instanceof HTMLElement &&
        !onTopPath.has(sibling)
      ) {
        markInert(sibling, owner);
      }
    }
    node = parent;
  }
}

function markInert(element: HTMLElement, owner: string) {
  const owners = readOwners(element);
  // The application owns this one; leave it (and its lack of a marker) alone.
  if (owners.length === 0 && element.hasAttribute("inert")) {
    return;
  }
  if (!owners.includes(owner)) {
    owners.push(owner);
    element.setAttribute(INERT_OWNER_ATTR, owners.join(" "));
  }
  element.setAttribute("inert", "");
}

type FocusTrapOptions = {
  // Mark the content outside the trap inert while it is installed, attributing every mark to
  // this owner id — the same id `releaseInertBackground` unwinds by. The caller owns the id so
  // that marking and unwinding cannot disagree about it. Omitted leaves the background alone: a
  // modal dialog wants it, a popover that only borrows focus does not.
  inertBackground?: string | null;
};

class FocusTrap {
  private container: HTMLElement;
  private restoreFocusElement: HTMLElement;
  private nodeWalker: TreeWalker;
  private inertOwner: string | null = null;

  constructor(container: HTMLElement, options?: FocusTrapOptions) {
    this.container = container;
    this.restoreFocusElement = document.activeElement as HTMLElement;
    this.nodeWalker = document.createTreeWalker(
      this.container,
      NodeFilter.SHOW_ELEMENT,
      {
        acceptNode: (node) => {
          if (node instanceof HTMLElement && focusable(node)) {
            return NodeFilter.FILTER_ACCEPT;
          }
          return NodeFilter.FILTER_SKIP;
        },
      }
    );
    if (options?.inertBackground) {
      this.inertOwner = options.inertBackground;
      inertBackgroundFor(this.inertOwner, container);
    }
    this.focusNext();
    this.container.addEventListener("keydown", (event) => {
      if (event.key === "Tab") {
        if (event.shiftKey) {
          this.focusPrevious();
        } else {
          this.focusNext();
        }
        event.preventDefault();
      }
    });
  }

  remove() {
    // Release the background before restoring focus: the element focus returns to is
    // usually one of the elements that was just made inert.
    if (this.inertOwner !== null) {
      releaseInert(this.inertOwner);
      this.inertOwner = null;
    }
    // The opener may have been unmounted while the dialog was open — routine when the
    // dialog's action re-renders the view behind it. Focusing a detached node drops
    // focus to `<body>` and loses the keyboard position, so fall back to the main
    // landmark instead.
    if (this.restoreFocusElement?.isConnected) {
      this.restoreFocusElement.focus();
      return;
    }
    const main = document.querySelector<HTMLElement>("main");
    if (main) {
      // `<main>` is not focusable on its own. Make it focusable, and take that back once focus
      // moves on — removing the attribute while it still holds focus would blur it immediately,
      // and leaving it behind would make the landmark click-focusable for good.
      if (!main.hasAttribute("tabindex")) {
        main.setAttribute("tabindex", "-1");
        main.addEventListener("blur", () => main.removeAttribute("tabindex"), {
          once: true,
        });
      }
      main.focus();
    }
  }

  focusChild(child: HTMLElement) {
    // Focus the element
    child.focus();
  }

  focusNext() {
    // Move to the next focusable element
    const nextNode = this.nodeWalker.nextNode() as HTMLElement | null;
    if (nextNode) {
      this.focusChild(nextNode);
    } else {
      // Try to reset to the first focusable element
      this.nodeWalker.currentNode = this.container;
      const nextNode = this.nodeWalker.nextNode() as HTMLElement | null;
      if (nextNode) {
        this.focusChild(nextNode);
      }
    }
  }

  focusPrevious() {
    // Move to the previous focusable element
    const previousNode = this.nodeWalker.previousNode() as HTMLElement | null;
    if (previousNode) {
      this.focusChild(previousNode);
    } else {
      // Try to reset to the last focusable element
      this.nodeWalker.currentNode = this.container;
      const lastNode = this.nodeWalker.lastChild() as HTMLElement | null;
      if (lastNode) {
        this.focusChild(lastNode);
      }
    }
  }
}

declare global {
  interface Window {
    createFocusTrap: (
      container: HTMLElement,
      options?: FocusTrapOptions
    ) => FocusTrap;
    releaseInertBackground: (owner: string) => void;
  }
}

window.createFocusTrap = (
  container: HTMLElement,
  options?: FocusTrapOptions
) => {
  return new FocusTrap(container, options);
};

// Exposed separately so a dialog that was unmounted while open can still unwind what it
// marked, when there is no trap left to call `remove()` on.
window.releaseInertBackground = (owner: string) => {
  releaseInert(owner);
};

export {};
