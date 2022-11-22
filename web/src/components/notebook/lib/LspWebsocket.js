import { VDoc } from "./VDoc"
import cookie from "react-cookies"
import { lspWsUrl } from "../../../store/config"
import { teamId, region, getProjectId } from "../../../store/cookie"
import {sliceDocumentString} from "../../../utils"
import PubSub from "pubsub-js";

// using task queue is more comfortable

class LspWebsocket {
  initWs(endpoint, lsp) {
    lsp.didOpenFile = {} // 重连后，清空缓存的didOpen文件
    lsp.ws = new WebSocket(endpoint)
    lsp.heartCheck = {
      timeout: 3000, //ms
      timeoutObj: null,
      reset: function () {
        clearTimeout(this.timeoutObj)
        this.start()
      },
      start: function () {
        this.timeoutObj = setTimeout(function () {
          lsp.safeSend("ping")
        }, this.timeout)
      },
    }
    // why must using lambda????
    lsp.ws.onmessage = (e) => {
      lsp.heartCheck.reset()
      if (e.data == "pong") {
        // handle heartbeat response
        // console.log("got 'pong'");
      } else {
        lsp.wsMessage(e)
      }
    }
    lsp.ws.onopen = (e) => {
      this.initialize(this.fresh, this.wsRootPath)
      lsp.heartCheck.start()
    }

    lsp.ws.onclose = (e) => {
      setTimeout(function () {
        lsp.initWs(endpoint, lsp)
      }, 1000)
    }

    lsp.ws.onerror = (e) => {
      lsp.ws.close()
    }

    LspWebsocket.instance = this
  }

  constructor() {
    let projectId = getProjectId()
    if (!projectId) return null
    if (!LspWebsocket.instance && region) {
      let endpoint = `${lspWsUrl}${teamId}_${projectId}`
      this.wsRootPath = "/tmp/halo-lsp/"
      this.endpoint = endpoint
      this.inited = false

      this.didOpenFile = {} // 缓存didOpen文件， 重连后，清空
      this.initWs(endpoint, this)
      this.vdocs = {}
      this.gotoDefine = {}
      this.completeHint = {}
    } else {
      return LspWebsocket.instance
    }
  }

  safeSend(message) {
    //readyState: CONNECTING/OPEN/CLOSING/CLOSED
    if (this.ws && this.ws.readyState == WebSocket.OPEN) {
      this.ws.send(message)
    } else {
      // console.log("Lsp websocket is " + this.ws.readyState);
    }
  }

  setVdoc(uri, vdoc) {
    this.vdocs[uri] = vdoc
  }
  getVdoc(path) {
    return this.vdocs[this.path2uri(path)]
  }

  uri2path(uri) {
    if (uri.startsWith("file://")) {
      return uri.substring(7);
    } else {
      return uri;
    }
  }

  path2uri(path) {
    if (path.startsWith("/")) {
      // only handle local file path
      return "file://" + path
    } else {
      return path // don't handle other path
    }
  }

  initialize(fresh, rootPath) {
    // console.log('open websocket ...');
    //const msg = {"jsonrpc":"2.0","id":0,"method":"initialize","params":{"processId":88985,"clientInfo":{"name":"Visual Studio Code","version":"1.61.0"},"locale":"zh-cn","rootPath":rootPath,"rootUri":this.path2uri(rootPath),"capabilities":{"workspace":{"applyEdit":true,"workspaceEdit":{"documentChanges":true,"resourceOperations":["create","rename","delete"],"failureHandling":"textOnlyTransactional","normalizesLineEndings":true,"changeAnnotationSupport":{"groupsOnLabel":true}},"didChangeConfiguration":{"dynamicRegistration":true},"didChangeWatchedFiles":{"dynamicRegistration":true},"symbol":{"dynamicRegistration":true,"symbolKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]},"tagSupport":{"valueSet":[1]}},"codeLens":{"refreshSupport":true},"executeCommand":{"dynamicRegistration":true},"configuration":true,"workspaceFolders":true,"semanticTokens":{"refreshSupport":true},"fileOperations":{"dynamicRegistration":true,"didCreate":true,"didRename":true,"didDelete":true,"willCreate":true,"willRename":true,"willDelete":true}},"textDocument":{"publishDiagnostics":{"relatedInformation":true,"versionSupport":false,"tagSupport":{"valueSet":[1,2]},"codeDescriptionSupport":true,"dataSupport":true},"synchronization":{"dynamicRegistration":true,"willSave":true,"willSaveWaitUntil":true,"didSave":true},"completion":{"dynamicRegistration":true,"contextSupport":true,"completionItem":{"snippetSupport":true,"commitCharactersSupport":true,"documentationFormat":["markdown","plaintext"],"deprecatedSupport":true,"preselectSupport":true,"tagSupport":{"valueSet":[1]},"insertReplaceSupport":true,"resolveSupport":{"properties":["documentation","detail","additionalTextEdits"]},"insertTextModeSupport":{"valueSet":[1,2]}},"completionItemKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25]}},"hover":{"dynamicRegistration":true,"contentFormat":["markdown","plaintext"]},"signatureHelp":{"dynamicRegistration":true,"signatureInformation":{"documentationFormat":["markdown","plaintext"],"parameterInformation":{"labelOffsetSupport":true},"activeParameterSupport":true},"contextSupport":true},"definition":{"dynamicRegistration":true,"linkSupport":true},"references":{"dynamicRegistration":true},"documentHighlight":{"dynamicRegistration":true},"documentSymbol":{"dynamicRegistration":true,"symbolKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]},"hierarchicalDocumentSymbolSupport":true,"tagSupport":{"valueSet":[1]},"labelSupport":true},"codeAction":{"dynamicRegistration":true,"isPreferredSupport":true,"disabledSupport":true,"dataSupport":true,"resolveSupport":{"properties":["edit"]},"codeActionLiteralSupport":{"codeActionKind":{"valueSet":["","quickfix","refactor","refactor.extract","refactor.inline","refactor.rewrite","source","source.organizeImports"]}},"honorsChangeAnnotations":false},"codeLens":{"dynamicRegistration":true},"formatting":{"dynamicRegistration":true},"rangeFormatting":{"dynamicRegistration":true},"onTypeFormatting":{"dynamicRegistration":true},"rename":{"dynamicRegistration":true,"prepareSupport":true,"prepareSupportDefaultBehavior":1,"honorsChangeAnnotations":true},"documentLink":{"dynamicRegistration":true,"tooltipSupport":true},"typeDefinition":{"dynamicRegistration":true,"linkSupport":true},"implementation":{"dynamicRegistration":true,"linkSupport":true},"colorProvider":{"dynamicRegistration":true},"foldingRange":{"dynamicRegistration":true,"rangeLimit":5000,"lineFoldingOnly":true},"declaration":{"dynamicRegistration":true,"linkSupport":true},"selectionRange":{"dynamicRegistration":true},"callHierarchy":{"dynamicRegistration":true},"semanticTokens":{"dynamicRegistration":true,"tokenTypes":["namespace","type","class","enum","interface","struct","typeParameter","parameter","variable","property","enumMember","event","function","method","macro","keyword","modifier","comment","string","number","regexp","operator"],"tokenModifiers":["declaration","definition","readonly","static","deprecated","abstract","async","modification","documentation","defaultLibrary"],"formats":["relative"],"requests":{"range":true,"full":{"delta":true}},"multilineTokenSupport":false,"overlappingTokenSupport":false},"linkedEditingRange":{"dynamicRegistration":true}},"window":{"showMessage":{"messageActionItem":{"additionalPropertiesSupport":true}},"showDocument":{"support":true},"workDoneProgress":true},"general":{"regularExpressions":{"engine":"ECMAScript","version":"ES2020"},"markdown":{"parser":"marked","version":"1.1.0"}}},"trace":"off","workspaceFolders":[{"uri":this.path2uri(rootPath),"name":"python"}]}};
    const msg = {
      jsonrpc: "2.0",
      id: 0,
      method: "initialize",
      params: {
        processId: 88985,
        rootPath: null,
        rootUri: null,
        capabilities: {
          workspace: {
            applyEdit: true,
            workspaceEdit: {
              documentChanges: true,
              resourceOperations: ["create", "rename", "delete"],
              failureHandling: "textOnlyTransactional",
            },
            didChangeConfiguration: { dynamicRegistration: true },
            didChangeWatchedFiles: { dynamicRegistration: true },
            symbol: {
              dynamicRegistration: true,
              symbolKind: {
                valueSet: [
                  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                  19, 20, 21, 22, 23, 24, 25, 26,
                ],
              },
              tagSupport: { valueSet: [1] },
            },
            codeLens: { refreshSupport: true },
            executeCommand: { dynamicRegistration: true },
            configuration: true,
            semanticTokens: { refreshSupport: true },
            fileOperations: {
              dynamicRegistration: true,
              didCreate: true,
              didRename: true,
              didDelete: true,
              willCreate: true,
              willRename: true,
              willDelete: true,
            },
            workspaceFolders: true,
          },
          textDocument: {
            publishDiagnostics: {
              relatedInformation: true,
              versionSupport: false,
              tagSupport: { valueSet: [1, 2] },
            },
            synchronization: {
              dynamicRegistration: true,
              willSave: true,
              willSaveWaitUntil: true,
              didSave: true,
            },
            completion: {
              dynamicRegistration: true,
              contextSupport: true,
              completionItem: {
                snippetSupport: true,
                commitCharactersSupport: true,
                documentationFormat: ["markdown", "plaintext"],
                deprecatedSupport: true,
                preselectSupport: true,
                insertReplaceSupport: true,
                tagSupport: { valueSet: [1] },
                resolveSupport: {
                  properties: [
                    "documentation",
                    "detail",
                    "additionalTextEdits",
                  ],
                },
                insertTextModeSupport: { valueSet: [1, 2] },
              },
              completionItemKind: {
                valueSet: [
                  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                  19, 20, 21, 22, 23, 24, 25,
                ],
              },
            },
            hover: {
              dynamicRegistration: true,
              contentFormat: ["markdown", "plaintext"],
            },
            signatureHelp: {
              dynamicRegistration: true,
              contextSupport: true,
              signatureInformation: {
                documentationFormat: ["markdown", "plaintext"],
                activeParameterSupport: false,
                parameterInformation: { labelOffsetSupport: true },
              },
            },
            definition: { dynamicRegistration: true },
            references: { dynamicRegistration: true },
            documentHighlight: { dynamicRegistration: true },
            documentSymbol: {
              dynamicRegistration: true,
              symbolKind: {
                valueSet: [
                  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                  19, 20, 21, 22, 23, 24, 25, 26,
                ],
              },
              hierarchicalDocumentSymbolSupport: true,
              tagSupport: { valueSet: [1] },
            },
            codeAction: {
              dynamicRegistration: true,
              isPreferredSupport: true,
              disabledSupport: true,
              dataSupport: true,
              honorsChangeAnnotations: false,
              resolveSupport: { properties: ["edit"] },
              codeActionLiteralSupport: {
                codeActionKind: {
                  valueSet: [
                    "",
                    "quickfix",
                    "refactor",
                    "refactor.extract",
                    "refactor.inline",
                    "refactor.rewrite",
                    "source",
                    "source.organizeImports",
                  ],
                },
              },
            },
            codeLens: { dynamicRegistration: true },
            formatting: { dynamicRegistration: true },
            rangeFormatting: { dynamicRegistration: true },
            onTypeFormatting: { dynamicRegistration: true },
            rename: { dynamicRegistration: true, prepareSupport: true },
            documentLink: { dynamicRegistration: true, tooltipSupport: true },
            typeDefinition: { dynamicRegistration: true },
            implementation: { dynamicRegistration: true },
            declaration: { dynamicRegistration: true },
            colorProvider: { dynamicRegistration: true },
            foldingRange: {
              dynamicRegistration: true,
              rangeLimit: 5000,
              lineFoldingOnly: true,
            },
            selectionRange: { dynamicRegistration: true },
            callHierarchy: { dynamicRegistration: true },
            semanticTokens: {
              dynamicRegistration: true,
              tokenTypes: [
                "namespace",
                "type",
                "class",
                "enum",
                "interface",
                "struct",
                "typeParameter",
                "parameter",
                "variable",
                "property",
                "enumMember",
                "event",
                "function",
                "method",
                "macro",
                "keyword",
                "modifier",
                "comment",
                "string",
                "number",
                "regexp",
                "operator",
              ],
              tokenModifiers: [
                "declaration",
                "definition",
                "readonly",
                "static",
                "deprecated",
                "abstract",
                "async",
                "modification",
                "documentation",
                "defaultLibrary",
              ],
              formats: ["relative"],
              requests: { range: true, full: { delta: true } },
              multilineTokenSupport: false,
              overlappingTokenSupport: false,
            },
            linkedEditingRange: { dynamicRegistration: true },
          },
          window: {
            showMessage: {
              messageActionItem: { additionalPropertiesSupport: false },
            },
            showDocument: { support: false },
            workDoneProgress: true,
          },
          general: {
            regularExpressions: { engine: "ECMAScript", version: "ES2020" },
            markdown: { parser: "marked", version: "1.1.0" },
          },
        },
        trace: "on",
        workspaceFolders: null,
        locale: "zh_cn",
        clientInfo: { name: "coc.nvim", version: "0.0.80" },
        workDoneToken: "41c586b8-6b12-4f53-a3ed-888fd592a547",
      },
    }
    this.safeSend(JSON.stringify(msg))
    // this.fresh = false;
    // this.safeSend('BAIHAI_RESET_ENV');
    this.inited = true
    this.ws.onopen = null
  }

  initialized() {
    if (!this.completeInit()) {
      const initialized = { jsonrpc: "2.0", method: "initialized", params: {} }
      this.safeSend(JSON.stringify(initialized))
      this.inited = true
    }
  }

    wsMessage (e) {
        // console.log('get message');
        const msg = JSON.parse(e.data);
        // console.log(msg);
        if (msg['result'] && msg['result']['capabilities']) {
            this.initialized();
        } else if (msg['result'] && msg['result']['isIncomplete'] != null ) {
            this.handleCompletion(msg);
        } else if(msg['result'] && msg['result']['documentation'] != null){
            this.handlerCompletionItem(msg)
        } else if (msg['method'] && msg['method'] === 'telemetry/event') {
          const message = msg['params']['message'];
          if ('REOPEN' === message) {
            this.didOpenFile[this.uri2path(msg['params']['uri'])] = false
          } else if ('OPENED' === message) {
            this.didOpenFile[this.uri2path(msg['params']['uri'])] = true
          }
        } else if (msg['method'] && msg['method'] === 'textDocument/publishDiagnostics' ) {
            this.handleDiagnostics(msg);
        } else if (msg['jsonrpc'] && msg['result'] && msg['id'] === this.gotoDefine.id) {
            this.handleGogoDefine(msg);
        // } else if (msg['result'] && msg['result'][0] && msg['result'][0].kind === 'quickfix') { // ugly
        } else if (msg['id'] == cookie.load('quickfixId')  ) { // ugly
            this.handleQuickfix(msg);
        } else if (msg.error) {
            const reInit = msg.error.message.match(/FeatureAlreadyRegisteredError/);
            if (reInit) {
                this.inited = true;
            }
        } else {
            // console.log("server send to client other msg start --------");
            console.log(msg);
            // console.log("server send to client other msg end   --------");
        }
    };

    didChange(path, cellId, data, text) {
        const vdoc = this.getVdoc(this.path2uri(path));
        vdoc.version += 1;
        let version = vdoc.version;
        let start_line = data.from.line;
        let start_ch = data.from.ch;
        let end_line = data.to.line;
        let end_ch = data.to.ch;
        // let text = data.text;

        // console.log('didChange() text = ' + data.text);
        // console.log('para        text = ' + text);

        //send DidChange websocket msg to lsp server start
        let lspClientData = this.textDocumenDidChange(vdoc, cellId, version, vdoc.uri, start_line, start_ch, end_line, end_ch, text);
        this.safeSend(lspClientData);
        //send DidChange websocket msg to lsp server end
  }

  didOpen(path, content, storeStat){
        const uri = this.path2uri(path);
        const tmp = this.textDocumentDidOpenObj(uri, content, storeStat);
        // console.log("create vdoc for " + tmp.vdoc.uri);
        this.setVdoc(uri, tmp.vdoc);
        const lspObj = tmp.lspObj;
        this.safeSend(JSON.stringify(lspObj));
  }

  handlerCompletionItem(msg){
    for (const [uri, vdoc] of Object.entries(this.vdocs)) {
      // response only contains id, no uri, so check with lastQueryId
      if (vdoc.lastQueryId() === msg.id) {
        const value = msg.result.documentation.value
        const sliceValue = sliceDocumentString(value)
        const label = msg.result.label
        const index = msg.result.data.index
        PubSub.publish('changeCompletionItemDom',{sliceValue,label,index})
      }
    }
  }

  handleCompletion(completionMessage) {
    //处理自动补全响应的结果"result": {"isIncomplete， Response， 响应 completion
    for (const [uri, vdoc] of Object.entries(this.vdocs)) {
      // response only contains id, no uri, so check with lastQueryId
      if (vdoc.lastQueryId() === completionMessage.id) {
        const arr = completionMessage.result.items
          .filter((item) => !item.label.startsWith("_"))
        vdoc.storeStat.lspKeywords = new Set(arr)
      }
    }
    this.completeHint.showHint &&
      this.completeHint.showHint(completionMessage.id)
  }

  handleDiagnostics(diagnosticMessage) {
    // console.log("server send to client Diagnostics start-----");
    //textDocument/publishDiagnostics， Notify， 通知代码哪些位置有哪些错误或警告
    // console.log(diagnosticMessage);
    const diagnostics = diagnosticMessage.params.diagnostics
    const vdoc = this.getVdoc(diagnosticMessage.params.uri)

    if (vdoc) {
      // console.log("vdoc is ok for :" + vdoc.uri);
      for (let diag of diagnostics) {
        const tmp = vdoc.vLineToCellLine(diag.range.start.line)
        diag.editorId = tmp.cellId
        diag.range.start.line = tmp.line
        diag.range.end.line = vdoc.vLineToCellLine(diag.range.end.line).line
      }
      const newDiags = diagnostics.filter(function (diag) {
        return !diag.message.includes("could not be resolved")
      })
      vdoc.storeStat.lspDiagnostics = new Set(diagnostics)
      vdoc.storeStat.diagnostics_time = new Date().getTime()
    } else {
      // todo
      // console.log("why vdoc is null ?? with: " + diagnosticMessage.params.uri);
    }

    // console.log("server send to client Diagnostics end-----");
  }

  completeInit() {
    return this.inited
  }

  close() {
    this.ws.close()
  }

  completionRequest(path, cellId, line, character) {
    let data = this.textDocumentCompletionRequest(path, cellId, line, character)
    this.safeSend(data)
  }

  codeActionRequest(
    id,
    uri,
    start_line,
    start_ch,
    end_line,
    end_ch,
    code,
    message,
    onlyvalue
  ) {
    let lspClientData = this.textDocumentCodeActionRequest(
      id,
      uri,
      start_line,
      start_ch,
      end_line,
      end_ch,
      code,
      message,
      onlyvalue
    )
    this.safeSend(lspClientData)
  }

  gotoDefineRequest(path, cellId, line, ch) {
    const uri = this.path2uri(path)
    const vdoc = this.getVdoc(uri)
    const id = vdoc.nextQueryId()
    this.gotoDefine = {
      id,
      path,
      cellId,
      search: {
        line,
        ch,
      },
    }
    const msg = {
      jsonrpc: "2.0",
      id: id,
      method: "textDocument/definition",
      params: {
        textDocument: {
          uri: uri,
        },
        position: {
          line: vdoc.cellLineToVline(cellId, line),
          character: ch,
        },
      },
    }
    this.safeSend(JSON.stringify(msg))
  }

  handleGogoDefine(data) {
    let result = []
    if (data.result && data.result.length > 0) {
      for (let item of data.result) {
        const vdoc = this.getVdoc(item.uri)
        if (undefined !== vdoc && item.range) {
          const { cellId, line } = vdoc.vLineToCellLine(item.range.start.line)
          item.cellId = cellId
          item.range.start.line = line
          item.range.end.line = line

          result.push(item)
        } else {
          result.push(item)
        }
      }
    }
    this.gotoDefine.result = result
  }

  textDocumentCompletionRequest(path, cellId, line, character) {
    const vdoc = this.getVdoc(this.path2uri(path))
    const messageId = vdoc.nextQueryId()
    this.completeHint.messageId = messageId //record the msgid of completion request

    const lspObj = {
      jsonrpc: "2.0",
      id: messageId,
      method: "textDocument/completion",
      params: {
        textDocument: { uri: vdoc.uri },
        position: {
          line: vdoc.cellLineToVline(cellId, line),
          character: character,
        },
        context: { triggerKind: 1 },
      },
    }

    let lspServerString = JSON.stringify(lspObj)
    console.log(lspServerString)
    return lspServerString
  }

  completionItemRequest(path,info,index) {
    // 最终返回一个json
    const {
      label,
      kind,
      workspacePath,
      filePath,
      position,
      symbolLabel,
    } = info
    const vdoc = this.getVdoc(this.path2uri(path))
    const messageId = vdoc.nextQueryId()

    const obj = {
      jsonrpc: "2.0",
      id:messageId,
      method: "completionItem/resolve",
      params: {
        label, // 从item中获取
        kind,  // 从item中获取
        data: {
          workspacePath, // 从item中获取
          filePath, // 从item中获取
          position, // 从item中获取
          symbolLabel, // 从item中获取
          index
        },
        insertText: "time($1)$0",
        insertTextFormat: 2,
      },
    }
    const jsonObj = JSON.stringify(obj)
    this.safeSend(jsonObj)
  }

  textDocumentDidOpenObj(uri, filecontent, storeStat) {
    let textDocument = {}
    const vdoc = new VDoc(uri, filecontent, storeStat)
    Object.defineProperties(textDocument, {
      uri: {
        value: uri,
        writable: true,
        enumerable: true,
      },
      languageId: {
        value: "python",
        writable: true,
        enumerable: true,
      },
      version: {
        value: 1, //每次第一次打开时，version为1
        writable: true,
        enumerable: true,
      },
      text: {
        value: vdoc.text,
        writable: true,
        enumerable: true,
      },
    })

    let params = {}
    Object.defineProperties(params, {
      textDocument: {
        value: textDocument,
        writable: true,
        enumerable: true,
      },
    })

    let lspObj = {}
    Object.defineProperties(lspObj, {
      jsonrpc: {
        value: "2.0",
        writable: true,
        enumerable: true,
      },
      method: {
        value: "textDocument/didOpen",
        writable: true,
        enumerable: true,
      },
      params: {
        value: params,
        writable: true,
        enumerable: true,
      },
    })

    // let lspServerString = JSON.stringify(lspObj);
    // console.log(lspServerString);
    return { lspObj: lspObj, vdoc: vdoc }

    // {
    //     "jsonrpc":"2.0",
    //     "method":"textDocument/didOpen",
    //     "params":{
    //         "textDocument":{
    //             "uri":"file:///Users/liuzhe/src/pythobn/kk.py",
    //             "languageId":"python",
    //             "version":1,
    //             "text":"\\n"
    //         }
    //     }
    // }
  }

  textDocumenDidChange(
    vdoc,
    cellid,
    version,
    uri,
    start_line,
    start_ch,
    end_line,
    end_ch,
    text
  ) {
    text = text + "" //change '' to "", because text need "" string , not [''];
    // {
    //     "jsonrpc":"2.0",
    //     "method":"textDocument/didChange",
    //     "params":{
    //         "textDocument":{
    //             "version":2,
    //             "uri":"file:///Users/liuzhe/src/pythobn/kk.py"
    //         },
    //         "contentChanges":[
    //             {
    //                 "range":{
    //                     "start":{
    //                         "line":0,
    //                         "character":0
    //                     },
    //                     "end":{
    //                         "line":0,
    //                         "character":0
    //                     }
    //                 },
    //                 "rangeLength":0,
    //                 "text":"i"
    //             }
    //         ]
    //     }
    // }
    let textDocument = { jsonrpc: "2.0", method: "textDocument/didChange" }
    textDocument.params = {
      textDocument: {
        version: version,
        uri: uri,
      },
    }

    let changes = [
      {
        range: {
          start: {
            line: start_line,
            character: start_ch,
          },
          end: {
            line: end_line,
            character: end_ch,
          },
        },
        text: text,
      },
    ]

    vdoc.adjustEdits(cellid, changes)
    textDocument.params.contentChanges = changes

    let lspServerString = JSON.stringify(textDocument)
    console.log(lspServerString)
    return lspServerString
  }

  getVlineDiagnostics(vdoc, data) {
    data.range.start.line = vdoc.cellLineToVline(
      data.editorId,
      data.range.start.line
    )
    data.range.end.line = vdoc.cellLineToVline(
      data.editorId,
      data.range.end.line
    )
  }

  textDocumentCodeActionRequest(vdoc, id, uri, data) {
    let cloneData
    if (Array.isArray(data)) {
      cloneData = data[0]
    } else {
      cloneData = data
    }

    this.getVlineDiagnostics(vdoc, cloneData) //modify its self
    let msg = {
      jsonrpc: "2.0",
      id: id,
      method: "textDocument/codeAction",
      params: {
        textDocument: {
          uri: uri,
        },
        range: {
          start: {
            line: cloneData.range.start.line,
            character: cloneData.range.start.character,
          },
          end: {
            line: cloneData.range.end.line,
            character: cloneData.range.end.character,
          },
        },
        context: {
          diagnostics: Array.isArray(data) ? data : [cloneData],
          only: ["quickfix"],
        },
      },
    }
    let lspServerString = JSON.stringify(msg)
    console.log(lspServerString)
    return lspServerString
  }

  handleQuickfix(msg) {
    // console.log("server send to client quick fix options msg --->" + msg)
    for (const [uri, vdoc] of Object.entries(this.vdocs)) {
      if (vdoc.lastQueryId() === msg.id) {
        if (!msg.result) return
        for (const result of msg.result) {
          for (const arg of result.command.arguments) {
            if (arg.edits) {
              for (let edit of arg.edits) {
                const tmp = vdoc.vLineToCellLine(edit.range.start.line)
                edit.editorId = tmp.cellId
                edit.range.start.line = tmp.line
                edit.range.end.line = vdoc.vLineToCellLine(
                  edit.range.end.line
                ).line
              }
            }
          }
        }
        vdoc.storeStat.lspQuickFixWords = msg
      }
    }
    console.log("server send to client quick fix options msg --->" + msg)
  }
}
var cloneObj = function (obj) {
  var newObj = {}
  if (obj instanceof Array) {
    newObj = []
  }
  for (var key in obj) {
    var val = obj[key]
    //newObj[key] = typeof val === 'object' ? arguments.callee(val) : val; //arguments.callee 在哪一个函数中运行，它就代表哪个函数, 一般用在匿名函数中。
    newObj[key] = typeof val === "object" ? cloneObj(val) : val
  }
  return newObj
}

/// 导出
export { LspWebsocket }
