/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/**
 * Manages updates for a IP Protection panelView in a given browser window.
 */
export class IPProtectionPanel {
  static TAGNAME = "ipprotection-panel";
  static CONTENT_ELEMENT = "#PanelUI-ipprotection-content";
  static CUSTOM_ELEMENTS_SCRIPT =
    "chrome://browser/content/ipprotection/ipprotection-customelements.js";
  /**
   * Loads the ipprotection custom element script
   * into a given window.
   *
   * Called on IPProtection.init for a new browser window.
   *
   * @param {Window} window
   */
  static loadCustomElements(window) {
    Services.scriptloader.loadSubScriptWithOptions(
      IPProtectionPanel.CUSTOM_ELEMENTS_SCRIPT,
      {
        target: window,
        async: true,
      }
    );
  }

  state = {};
  panel = null;

  /**
   * Check the state of the enclosing panel to see if
   * it is active (open or showing).
   */
  get active() {
    let panelParent = this.panel?.closest("panel");
    if (!panelParent) {
      return false;
    }
    return panelParent.state == "open" || panelParent.state == "showing";
  }

  /**
   * Creates an instance of IPProtectionPanel for a specific browser window.
   *
   * Inserts the panel component customElements registry script.
   *
   * @param {Window} _window
   *   Window containing the panelView to manage.
   */
  constructor(_window) {
    this.handleEvent = this.#handleEvent.bind(this);
  }

  /**
   * Set the state for this panel.
   *
   * Updates the current panel component state,
   * if the panel is currently active (showing or not hiding).
   *
   * @param {object} state
   *    The state object from IPProtectionPanel.
   */
  setState(state) {
    this.state = state;
    if (this.active) {
      this.updateState(state);
    }
  }

  /**
   * Updates the state of the panel component.
   *
   * @param {object} state
   *   The state object from IPProtectionPanel.
   * @param {Element} panelEl
   *   The panelEl element to update the state on.
   */
  updateState(state = this.state, panelEl = this.panel) {
    if (!panelEl?.isConnected || !panelEl.state) {
      return;
    }

    Object.assign(panelEl.state, state);
  }

  /**
   * Updates the visibility of the panel components before they will shown.
   *
   * - If the panel component has already been created, updates the state.
   * - Creates a panel component if need, state will be updated on once it has
   *   been connected.
   *
   * @param {XULElement} panelView
   *   The panelView element from the CustomizableUI widget callback.
   */
  showing(panelView) {
    if (this.panel) {
      this.updateState();
    } else {
      this.createPanel(panelView);
    }
  }

  /**
   * Called when the panel elements will be hidden.
   *
   * Disables updates to the panel.
   */
  hiding() {}

  /**
   * Creates a panel component in a panelView.
   *
   * @param {XULBrowserElement} panelView
   */
  createPanel(panelView) {
    let { ownerDocument } = panelView;
    let contentEl = panelView.querySelector(IPProtectionPanel.CONTENT_ELEMENT);
    let panelEl = ownerDocument.createElement(IPProtectionPanel.TAGNAME);

    this.panel = panelEl;

    this.#addPanelListeners(ownerDocument);

    contentEl.appendChild(panelEl);
  }

  /**
   * Resets the state of the panel, removes listeners and disables updates.
   */
  destroy() {
    if (this.panel) {
      this.panel.remove();
      this.#removePanelListeners(this.panel.ownerDocument);
    }
    this.state = {};
  }

  #addPanelListeners(doc) {
    doc.addEventListener("IPProtection:Init", this.handleEvent);
  }

  #removePanelListeners(doc) {
    doc.removeEventListener("IPProtection:Init", this.handleEvent);
  }

  #handleEvent(event) {
    if (event.type == "IPProtection:Init") {
      this.updateState();
    }
  }
}
