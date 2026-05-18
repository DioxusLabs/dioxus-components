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

class FocusTrap {
  private container: HTMLElement;
  private restoreFocusElement: HTMLElement;
  private nodeWalker: TreeWalker;

  constructor(container: HTMLElement) {
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
    this.restoreFocusElement.focus();
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
    createFocusTrap: (container: HTMLElement) => FocusTrap;
  }
}

window.createFocusTrap = (container: HTMLElement) => {
  return new FocusTrap(container);
};

export {};
