import { createApp, h } from 'vue';
import { createInertiaApp, Link, Head } from '@inertiajs/inertia-vue3';
import { InertiaProgress } from '@inertiajs/progress';

InertiaProgress.init();

createInertiaApp({

    resolve: name => import(`../Pages/${name}.vue`),
    setup({ el, App, props, plugin }) {
        const app = createApp({
            render: () => h(App, props),
        });

        app.use(plugin);
        app.component('InertiaLink', Link);
        app.component('InertiaHead', Head);
        app.mount(el);
    },
}).catch(error => {
    console.error("Failed to create Inertia app:", error);
});

// Additional Check for element with ID 'app'
document.addEventListener("DOMContentLoaded", () => {
    const appElement = document.getElementById("app");
    if (!appElement) {
        console.error("Element with ID 'app' not found");
        return;
    }

    try {
        const pageData = appElement.dataset.page;
        if (!pageData) {
            console.error("'data-page' attribute not found on element with ID 'app'");
            return;
        }

        const parsedPageData = JSON.parse(pageData);
        console.log("Parsed page data:", parsedPageData);

        if (!parsedPageData.component) {
            console.error("Component not found in page data");
            return;
        }

        // Check if component exists and resolve correctly
        import(`../Pages/${parsedPageData.component}.vue`)
            .then(component => {
                console.log(`Component ${parsedPageData.component} loaded successfully.`);
            })
            .catch(error => {
                console.error(`Error loading component ${parsedPageData.component}:`, error);
            });

    } catch (error) {
        console.error("Error parsing 'data-page' attribute:", error);
    }
});
