/*
 * codeActionProvider.ts
 * Copyright (c) Microsoft Corporation.
 * Licensed under the MIT license.
 *
 * Handles 'code actions' requests from the client.
 */

import { CancellationToken, CodeAction, CodeActionKind, Command } from 'vscode-languageserver';
import { TextEdit } from 'vscode-languageserver-protocol';

import { Commands } from '../commands/commands';
import { throwIfCancellationRequested } from '../common/cancellationUtils';
import {
    AddMissingOptionalToParamAction,
    CreateTypeStubFileAction,
    Diagnostic,
    SingleChanges,
} from '../common/diagnostic';
import { Range } from '../common/textRange';
import { WorkspaceServiceInstance } from '../languageServerBase';
import { Localizer } from '../localization/localize';

export class CodeActionProvider {
    static placeHolder = '#^_^#';
    static async getCodeActionsForPosition(
        workspace: WorkspaceServiceInstance,
        filePath: string,
        range: Range,
        token: CancellationToken
    ) {
        throwIfCancellationRequested(token);

        const codeActions: CodeAction[] = [];

        if (!workspace.disableLanguageServices) {
            const diags = await workspace.serviceInstance.getDiagnosticsForRange(filePath, range, token);
            const typeStubDiag = diags.find((d) => {
                const actions = d.getActions();
                return actions && actions.find((a) => a.action === Commands.createTypeStub);
            });

            if (typeStubDiag) {
                const action = typeStubDiag
                    .getActions()!
                    .find((a) => a.action === Commands.createTypeStub) as CreateTypeStubFileAction;
                if (action) {
                    const createTypeStubAction = CodeAction.create(
                        Localizer.CodeAction.createTypeStubFor().format({ moduleName: action.moduleName }),
                        Command.create(
                            Localizer.CodeAction.createTypeStub(),
                            Commands.createTypeStub,
                            workspace.rootPath,
                            action.moduleName,
                            filePath
                        ),
                        CodeActionKind.QuickFix
                    );
                    codeActions.push(createTypeStubAction);
                }
            }

            const addOptionalDiag = diags.find((d) => {
                const actions = d.getActions();
                return actions && actions.find((a) => a.action === Commands.addMissingOptionalToParam);
            });

            if (addOptionalDiag) {
                const action = addOptionalDiag
                    .getActions()!
                    .find((a) => a.action === Commands.addMissingOptionalToParam) as AddMissingOptionalToParamAction;
                if (action) {
                    const addMissingOptionalAction = CodeAction.create(
                        Localizer.CodeAction.addOptionalToAnnotation(),
                        Command.create(
                            Localizer.CodeAction.addOptionalToAnnotation(),
                            Commands.addMissingOptionalToParam,
                            action.offsetOfTypeNode
                        ),
                        CodeActionKind.QuickFix
                    );
                    codeActions.push(addMissingOptionalAction);
                }
            }
            /* other new actions */

            /* fix:# "xxx" is not defined #           x = sys.path       */

            const edits: TextEdit[] = [
                {
                    range: { start: { line: 0, character: 0 }, end: { line: 0, character: 0 } },
                    newText: 'import ' + this.placeHolder + '\n',
                },
            ];
            this.simChangeInOneFile(
                "Add Missing '" + this.placeHolder + "'",
                null,
                workspace,
                filePath,
                range,
                token,
                codeActions,
                /"(.*)" is not defined/,
                1,
                diags,
                edits
            );

            /* fix:# Expected "(" #    def foo:  */

            this.simChangeInOneFile(
                'Add Expected "("',
                null,
                workspace,
                filePath,
                range,
                token,
                codeActions,
                /Expected "\("/,
                15,
                diags,
                [{ range: { start: range.start, end: range.start }, newText: '(' }]
            );

            /* fix:# "Statements must be separated by newlines or semicolons" #  y = 3 5  */
            this.simChangeInOneFile(
                'Add "," ',
                null,
                workspace,
                filePath,
                range,
                token,
                codeActions,
                /Statements must be separated by newlines or semicolons/,
                15,
                diags,
                [{ range: { start: range.start, end: range.start }, newText: ',' }]
            );

            /* fix:# Expected expression #   if (True && True) */
            this.simChangeInOneFile(
                'Change to "and"',
                /&/,
                workspace,
                filePath,
                range,
                token,
                codeActions,
                /Expected expression/,
                15,
                diags,
                [
                    {
                        range: {
                            start: { line: range.start.line, character: range.start.character - 1 },
                            end: range.end,
                        },
                        newText: ' and ',
                    },
                ]
            );

            /* fix:# Expected expression #   if (True || True) */
            this.simChangeInOneFile(
                'Change to "or"',
                /\|/,
                workspace,
                filePath,
                range,
                token,
                codeActions,
                /Expected expression/,
                15,
                diags,
                [
                    {
                        range: {
                            start: { line: range.start.line, character: range.start.character - 1 },
                            end: range.end,
                        },
                        newText: ' or ',
                    },
                ]
            );

            /* fix:# Expected expression #   3 === 5 */
            this.simChangeInOneFile(
                'Change to "=="',
                /=/,
                workspace,
                filePath,
                range,
                token,
                codeActions,
                /Expected expression/,
                15,
                diags,
                [
                    {
                        range: range,
                        newText: '',
                    },
                ]
            );

            /* add break point to see diagnostic */
            const x = 3;
        }

        return codeActions;
    }

    static matchAndReplace(src: string, template: string, regex: RegExp, matchIndex: number) {
        const matches = src.match(regex);
        if (matches) {
            const matchPart = matches[matchIndex];
            const ret = matchPart ? template.replace(this.placeHolder, matchPart) : template;
            return { ret: ret, capture: matchPart };
        } else {
            return { ret: template, capture: null };
        }
    }

    static simChangeInOneFile(
        titleTemplate: string,
        expectRangeRe: RegExp | null,
        workspace: WorkspaceServiceInstance,
        filePath: string,
        range: Range,
        token: CancellationToken,
        codeActions: CodeAction[],
        regex: RegExp,
        matchIndex: number,
        diags: Diagnostic[],
        soloEdits: TextEdit[]
    ) {
        const aimDiag = diags.find((d) => {
            return d.message.match(regex);
        });

        const rangeString = workspace.serviceInstance.getTextOnRange(filePath, range, token);
        const stringIsConsiderd = expectRangeRe === null || rangeString?.match(expectRangeRe);

        if (aimDiag && stringIsConsiderd) {
            const { ret: title, capture: matchPart } = this.matchAndReplace(
                aimDiag.message,
                titleTemplate,
                regex,
                matchIndex
            );

            if (matchPart) {
                soloEdits.forEach((edit) => (edit.newText = edit.newText.replace(this.placeHolder, matchPart)));
            }

            const singleChange: SingleChanges = {
                action: title,
                filePath: filePath,
                edits: soloEdits,
            };

            const aac = CodeAction.create(
                title,
                Command.create(title, Commands.singleChange, singleChange),
                CodeActionKind.QuickFix
            );
            codeActions.push(aac);
        }
    }
}
