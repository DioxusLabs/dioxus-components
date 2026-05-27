The Sidebar component is a vertical interface panel fixed to the screen edge, displaying navigation menus or filters that enable quick access to different sections of an application.

## Component Structure

```rust
// Provider: supplies open/side/collapsible signals and ⌘/Ctrl+B toggle
SidebarProvider {
    Sidebar {
        side: SidebarSide::Left,                     // left/right placement
        variant: SidebarVariant::Sidebar,            // chrome: Sidebar | Floating | Inset
        collapsible: SidebarCollapsible::Offcanvas,  // behavior: Offcanvas | Icon | None

        // Layout - Header
        SidebarHeader {
            SidebarTrigger {}                        // toggle button (as)
        }

        // Layout - Scrollable content area
        SidebarContent {
            SidebarGroup {
                SidebarGroupLabel { "..." }          // optional label (as)
                SidebarGroupAction { "..." }         // optional action (as)
                SidebarGroupContent {                // wraps menus
                    SidebarMenu {
                        SidebarMenuItem {
                            SidebarMenuButton {      // primary item (as)
                                is_active: true,     // highlight state
                                tooltip: rsx!("..."),// Option<Element>; wraps tooltip only when Some
                                Icon {}              // icon node
                                span { "..." }       // text node
                            }
                            SidebarMenuAction { show_on_hover: true, Icon {} } // trailing action (as)
                            SidebarMenuBadge { "+..." }                        // optional badge
                        }
                        SidebarMenuItem {            // nested submenu
                            SidebarMenuSub {
                                SidebarMenuSubItem {
                                    SidebarMenuSubButton { "..." } // submenu button/link (as)
                                }
                            }
                        }
                    }
                }
            }
        }

        // Layout -  Footer
        SidebarFooter {
            SidebarMenu { /* ... */ }
        }
    }

    // Optional desktop rail controller placed between rail and content
    SidebarRail {}                               // draggable resize handle

    // Layout - Main content area beside the rail
    SidebarInset { /* ... */ }
}
```

## Behaviors
- Layout: `variant` adjusts chrome (`Floating/Inset` adds padding/radius); `side` selects left/right.
- Collapse: `Offcanvas` hides the rail; `Icon` keeps a thin icon strip and hides labels/actions/badges; `None` is static.
- Keyboard: ⌘/Ctrl+B toggles via provider. Focus rings are defined in `sidebar/style.css`; keep or replace with `:focus-visible` styles.
- Tooltips: `tooltip: Option<Element>` on `SidebarMenuButton`; `None` skips wrapping in Tooltip.

## Custom Rendering with `as`
Supported components: `SidebarTrigger`, `SidebarGroupLabel`, `SidebarGroupAction`, `SidebarMenuButton`, `SidebarMenuAction`, `SidebarMenuSubButton`. Use `as: |attrs| rsx! { ... }` and spread `..attrs` to retain merged attributes, state data, and handlers.
