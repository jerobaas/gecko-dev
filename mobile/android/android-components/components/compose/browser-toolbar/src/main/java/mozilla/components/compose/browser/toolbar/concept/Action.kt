/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.components.compose.browser.toolbar.concept

import android.graphics.drawable.Drawable
import androidx.annotation.DrawableRes
import androidx.annotation.StringRes
import mozilla.components.compose.browser.toolbar.store.BrowserToolbarInteraction
import mozilla.components.compose.browser.toolbar.store.BrowserToolbarInteraction.BrowserToolbarEvent
import mozilla.components.compose.browser.toolbar.store.BrowserToolbarInteraction.BrowserToolbarMenu
import mozilla.components.ui.icons.R as iconsR

/**
 * Actions that can be added to the toolbar.
 */
sealed class Action {
    /**
     * An action button to be added to the toolbar that can be configures with resource ids.
     *
     * @property drawableResId The icon resource to be displayed for the action button.
     * @property contentDescription The content description for the action button.
     * @property state the current [State] of the action button.
     * @property highlighted Whether or not to highlight this button.
     * @property onClick [BrowserToolbarInteraction] describing how to handle this button being clicked.
     * @property onLongClick Optional [BrowserToolbarInteraction] describing how to handle this button
     * being long clicked.
     */
    data class ActionButtonRes(
        @DrawableRes val drawableResId: Int,
        @StringRes val contentDescription: Int,
        val state: ActionButton.State = ActionButton.State.DEFAULT,
        val highlighted: Boolean = false,
        val onClick: BrowserToolbarInteraction,
        val onLongClick: BrowserToolbarInteraction? = null,
    ) : Action()

    /**
     * An action button to be added to the toolbar.
     *
     * @property drawable A [Drawable] to use as icon for this button.
     * @property shouldTint Whether or not to apply the application default tint to this icon.
     * @property contentDescription A [String] to use as content description for this button.
     * @property state the current [State] of the action button.
     * @property highlighted Whether or not to highlight this button.
     * @property onClick [BrowserToolbarInteraction] describing how to handle this button being clicked.
     * @property onLongClick Optional [BrowserToolbarInteraction] describing how to handle this button
     * being long clicked.
     */
    data class ActionButton(
        val drawable: Drawable?,
        val shouldTint: Boolean = true,
        val contentDescription: String,
        val state: State = State.DEFAULT,
        val highlighted: Boolean = false,
        val onClick: BrowserToolbarInteraction,
        val onLongClick: BrowserToolbarInteraction? = null,
    ) : Action() {

        /**
         * An enum class defining the current state of the [ActionButton].
         */
        enum class State {
            DEFAULT, DISABLED, ACTIVE,
        }
    }

    /**
     * An action button styled as a dropdown button to be added to the toolbar.
     * This wraps the provided [icon] or [iconResource] at the start with a down arrow to it's right
     * to indicate that clicking this will open a dropdown menu.
     *
     * @property icon A [Drawable] to use as icon for this button. If null will use [iconResource] instead.
     * @property iconResource The resource id of the icon to use for this button if a [Drawable] is not provided.
     * @property contentDescription The content description for this button.
     * @property menu The [BrowserToolbarMenu] to show when this button is clicked.
     */
    data class DropdownAction(
        val icon: Drawable?,
        @DrawableRes val iconResource: Int = iconsR.drawable.mozac_ic_star_fill_20,
        @StringRes val contentDescription: Int,
        val menu: BrowserToolbarMenu,
    ) : Action()

    /**
     * An action button styled as a tab counter to be added to the toolbar.
     * This shows the provided [count] number inside of a squircle if lower than 100, otherwise it will
     * show an infinity symbol inside of the same squircle shape.
     *
     * @property count The number of tabs to display in the tab counter.
     * @property contentDescription The content description for this button.
     * @property showPrivacyMask Whether ot not to decorate this button with a top right icon
     * signaling that the tabs are private.
     * @property onClick [BrowserToolbarEvent] to be dispatched when this button is clicked.
     * @property onLongClick Optional [BrowserToolbarInteraction] describing how to handle this button
     * being long clicked.
     */
    data class TabCounterAction(
        val count: Int,
        val contentDescription: String,
        val showPrivacyMask: Boolean,
        val onClick: BrowserToolbarEvent,
        val onLongClick: BrowserToolbarInteraction? = null,
    ) : Action()
}
