import { InfoBox } from "../floater/info_box.mjs";

// Example would be `#info:...`
const ACTION_MATCH_HREF = /^#([a-zA-Z][a-zA-Z0-9]*):(.*)$/;

/// Wraps the document object and provides a simple API for interacting with the DOM specifically
/// with items that need to be handled globally.
/// It is a singleton, exposed as bux_document.
export abstract class BuxDocument {
    /// If this is set, then any click that is not inside of it should result in closing it automatically
    private active_info_box_to_close_on_click: InfoBox | null = null;
    protected $body = document.body as HTMLBodyElement;

    constructor() {
        // global click handler
        this.$body.addEventListener("click", (event) => this.on_body_click(event));
    }

    /// When the body is clicked, this function is called to evaluate the bux-document-action attribute
    protected on_body_click(event: MouseEvent) {
        const $target = event.target as HTMLElement;

        let href = $target.getAttribute("href");

        // if there is an active info box to close on click, then close it
        if (this.active_info_box_to_close_on_click !== null && $target.closest(".InfoBox") === null) {
            // this should have the effect of setting active_info_box_to_close_on_click to null within the close handler
            this.active_info_box_to_close_on_click.close();
        }

        // outer if is for efficiency
        if (href !== null && href.startsWith("#")) {
            // eg: #verb:<data>
            let match = href.match(ACTION_MATCH_HREF);
            if (match !== null) {
                event.preventDefault();
                this.on_action({
                    $trigger: $target,
                    title: $target.getAttribute("title") || undefined,
                    verb: match[1],
                    data: match[2],
                });
            }
        }
    }

    protected on_action(action: { $trigger: HTMLElement; title?: string; verb: string; data: string }) {
        switch (action.verb) {
            // If the href = `#info:topic`, then:
            //   `action.verb = "info"`
            //   `action.data = "topic"`
            case "info":
                this.on_action_info({ $trigger: action.$trigger, title: action.title, topic: action.data });
                break;

            default:
                console.error("Unknown action verb", action);
        }
    }

    /// only call the super class if the topic is not recognized
    protected on_action_info(opts: { $trigger: HTMLElement; title?: string; topic: string }) {
        console.error("Unknown topic", opts);
    }

    /// function to show an InfoBox with the given html content near the target
    /// other info boxes will be destroyed
    protected show_info_box(opts: { $trigger: HTMLElement; title?: string; html?: string; text?: string }) {
        if (this.active_info_box_to_close_on_click !== null) {
            this.active_info_box_to_close_on_click.close();
        }

        if (!opts.title) {
            opts.title = "Information";
        }

        this.active_info_box_to_close_on_click = new InfoBox({
            $trigger: opts.$trigger,
            title: opts.title,
            html: opts.html,
            text: opts.text,
            on_close: () => {
                this.active_info_box_to_close_on_click = null;
            },
        });

        this.active_info_box_to_close_on_click.show();
    }
}
