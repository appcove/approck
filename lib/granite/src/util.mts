/// A specific type for HTML strings
export type HTML = string;

/// htmlspecialchars equivalent
/// powered by the he.encode() function
export function HS(str: string): HTML {
    // replace special chars
    return str.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(
        /'/g,
        "&#039;",
    );
}

export function QA(str: string): HTML {
    return "\"" + HS(str) + "\"";
}

export function SE<T extends HTMLElement>($search_element: HTMLElement | DocumentFragment, selector: string): T {
    const $found_element: T = $search_element.querySelector(selector) as T;
    if ($found_element === null) {
        console.error("Element not found for selector `", selector, "` within", $search_element);
        throw new Error(`Element not found for selector "${selector}"`);
    }
    return $found_element;
}

/// Takes arbitrary string input like "Welcome, Company, Inc.!" and turns it into "welcome-company-inc"
export function DASH(str: string): string {
    return str.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();
}

/// handy shortcut to creating an element with this syntax:
/// `elem("div.my-class", "inner html")`
export function make_element(tag_and_classes: string, inner_html: string): HTMLElement {
    const [tag, ...classes] = tag_and_classes.split(".");
    const element = document.createElement(tag);
    element.innerHTML = inner_html;
    classes.forEach(cls => element.classList.add(cls));
    return element;
}

/// Pass some CSS and get a style element back
export function make_style(css: string): HTMLStyleElement {
    const style = document.createElement("style");
    style.innerHTML = css;
    return style;
}
