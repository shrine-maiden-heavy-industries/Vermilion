/* SPDX-License-Identifier: BSD-3-Clause */
import {window} from 'vscode'

/* exported startSpinner */
export function startSpinner(message: string)
{
	window.setStatusBarMessage(`Vermilion: $(settings-gear~spin) ${message}`)
}

/* exported stopSpinner */
export function stopSpinner(message?: string)
{
	window.setStatusBarMessage(message ? `Vermilion: ${message}` : 'Vermilion')
}
