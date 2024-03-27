#include <wayland-server.h>
#include <wayland-client.h>

// Global variables
struct wl_display *display = NULL;
struct wl_compositor *compositor = NULL;
struct wl_surface *surface = NULL;
struct wl_shell *shell = NULL;
struct wl_shell_surface *shell_surface = NULL;

// Wayland shell surface listener
void handle_ping(void *data, struct wl_shell_surface *shell_surface, uint32_t serial) {
    wl_shell_surface_pong(shell_surface, serial);
}

static const struct wl_shell_surface_listener shell_surface_listener = {
    .ping = handle_ping,
};

// Create Wayland compositor
void create_compositor() {
    compositor = wl_display_get_compositor(display);
}

// Create Wayland surface
void create_surface() {
    surface = wl_compositor_create_surface(compositor);
}

// Create Wayland shell surface
void create_shell_surface() {
    shell_surface = wl_shell_get_shell_surface(shell, surface);
    wl_shell_surface_add_listener(shell_surface, &shell_surface_listener, NULL);
    wl_shell_surface_set_toplevel(shell_surface);
}

// Registry global callback
void registry_handle_global(void *data, struct wl_registry *registry, uint32_t name,
                            const char *interface, uint32_t version) {
    if (strcmp(interface, "wl_compositor") == 0) {
        create_compositor();
    } else if (strcmp(interface, "wl_shell") == 0) {
        shell = wl_registry_bind(registry, name, &wl_shell_interface, 1);
        create_shell_surface();
    }
}

// Registry global remove callback
void registry_handle_global_remove(void *data, struct wl_registry *registry, uint32_t name) {
    // No action needed in this example
}

// Wayland registry listener
static const struct wl_registry_listener registry_listener = {
    .global = registry_handle_global,
    .global_remove = registry_handle_global_remove,
};

int main() {
    // Connect to Wayland display
    display = wl_display_connect(NULL);
    if (!display) {
        fprintf(stderr, "Failed to connect to Wayland display\n");
        return 1;
    }

    // Create Wayland registry
    struct wl_registry *registry = wl_display_get_registry(display);
    wl_registry_add_listener(registry, &registry_listener, NULL);
    wl_display_roundtrip(display);

    // Main event loop
    while (wl_display_dispatch(display) != -1) {
        // Continue processing events
    }

    // Cleanup and exit
    wl_display_disconnect(display);
    return 0;
}
