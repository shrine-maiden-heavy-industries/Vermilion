/* SPDX-License-Identifier: BSD-3-Clause */
import {workspace, WorkspaceFolder} from 'vscode'
import * as path from 'path'
import {
	BaseLanguageClient,
	LanguageClientOptions,
	NotificationType,
} from 'vscode-languageclient'
import {
	DocumentSelector,
	LanguageClient,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node'
import {extensionContext, WorkspaceProgress} from './extension'
import {Observable} from './utils/observable'

export const documentSelector: DocumentSelector =
[
	{language: 'verilog', scheme: 'file'},
	{language: 'verilog', scheme: 'untitled'},
]

/* exported createLanguageClient */
export async function createLanguageClient(folder: WorkspaceFolder): Promise<LanguageClient>
{
	const clientOptions: LanguageClientOptions =
	{
		documentSelector,
		diagnosticCollectionName: 'vermilion',
		initializationOptions: workspace.getConfiguration('vermilion'),
		workspaceFolder: folder,
	}

	// XXX: This needs to actually go looking for Vermilion using either the configuration keys, or $PATH
	const languageServer = extensionContext.asAbsolutePath(path.join('build', 'server', 'vermilion'))

	const serverOptions: ServerOptions =
	{
		run:
		{
			command: languageServer,
			args: ['lang-server'],
			transport: TransportKind.pipe,
		},
		debug:
		{
			command: languageServer,
			args: ['-v', 'lang-server'],//, '--inspect=6009'],
			transport: TransportKind.pipe,
		},
	}

	return new LanguageClient(
		'vermilion-client',
		'Vermilion Language Server',
		serverOptions,
		clientOptions,
	)
}

interface ProgressParams
{
	id: string
	title?: string
	message?: string
	percentage?: number
	done?: boolean
}

/* exported getProgressMessage */
function getProgressMessage(progress: ProgressParams)
{
	if (typeof progress.percentage === 'number')
		return `${Math.round(progress.percentage * 100)}%`
	else if (progress.message)
		return progress.message
	else if (progress.title)
		return progress.title
	return ''
}

/* exported setupProgress */
export async function setupProgress(client: BaseLanguageClient, workspaceProgress: Observable<WorkspaceProgress>)
{
	const runningProgress: Set<string> = new Set()

	if (client.needsStart())
		await client.start()
	client.onNotification(
		new NotificationType<ProgressParams>('window/progress'),
		progress =>
		{
			if (progress.done)
				runningProgress.delete(progress.id)
			else
				runningProgress.add(progress.id)

			workspaceProgress.value = runningProgress.size ?
				{state: 'progress', message: getProgressMessage(progress)} :
				{state: 'ready'}
		},
	)
}
