// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="setup/getting_started.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="setup/ecal_installation.html"><strong aria-hidden="true">2.1.</strong> Install eCAL</a></li></ol></li><li class="chapter-item expanded "><a href="project_structure.html"><strong aria-hidden="true">3.</strong> Project Structure</a></li><li class="chapter-item expanded "><a href="types/message_types.html"><strong aria-hidden="true">4.</strong> Message Types</a></li><li class="chapter-item expanded "><a href="examples/index.html"><strong aria-hidden="true">5.</strong> Examples</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="examples/binary.html"><strong aria-hidden="true">5.1.</strong> Binary</a></li><li class="chapter-item expanded "><a href="examples/string.html"><strong aria-hidden="true">5.2.</strong> String</a></li><li class="chapter-item expanded "><a href="examples/protobuf.html"><strong aria-hidden="true">5.3.</strong> Protobuf</a></li><li class="chapter-item expanded "><a href="examples/json.html"><strong aria-hidden="true">5.4.</strong> JSON</a></li><li class="chapter-item expanded "><a href="examples/service_server.html"><strong aria-hidden="true">5.5.</strong> Server</a></li><li class="chapter-item expanded "><a href="examples/service_client.html"><strong aria-hidden="true">5.6.</strong> Client</a></li><li class="chapter-item expanded "><a href="examples/monitoring.html"><strong aria-hidden="true">5.7.</strong> Monitoring</a></li><li class="chapter-item expanded "><a href="examples/logging.html"><strong aria-hidden="true">5.8.</strong> Logging</a></li></ol></li><li class="chapter-item expanded "><a href="api/index.html"><strong aria-hidden="true">6.</strong> API Documentation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="api/ecal.html"><strong aria-hidden="true">6.1.</strong> Ecal Lifecycle</a></li><li class="chapter-item expanded "><a href="api/publisher.html"><strong aria-hidden="true">6.2.</strong> Typed Publisher</a></li><li class="chapter-item expanded "><a href="api/subscriber.html"><strong aria-hidden="true">6.3.</strong> Typed Subscriber</a></li><li class="chapter-item expanded "><a href="api/message_types.html"><strong aria-hidden="true">6.4.</strong> Supported Message Types</a></li><li class="chapter-item expanded "><a href="api/zero_copy_messaging.html"><strong aria-hidden="true">6.5.</strong> Zero Copy Messaging</a></li><li class="chapter-item expanded "><a href="api/service_server.html"><strong aria-hidden="true">6.6.</strong> Service Server</a></li><li class="chapter-item expanded "><a href="api/service_client.html"><strong aria-hidden="true">6.7.</strong> Service Client</a></li></ol></li><li class="chapter-item expanded "><a href="project_status.html"><strong aria-hidden="true">7.</strong> Project Status</a></li><li class="chapter-item expanded "><a href="about.html"><strong aria-hidden="true">8.</strong> About</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
