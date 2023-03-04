import React, {Fragment, useEffect, useRef, useState} from 'react'
import { configure } from 'mobx'; // 开启严格模式
import './index.less';
import Header from '@/idp/component/header';
import { Layout } from "antd";
import Content from "@/layout/content";
import LeftNav from "@/pages/common/leftNav";
import FooterBar from "@components/../layout/footer/FooterBar";
import { AliveScope } from 'react-activation';
import Terminal from '@/idp/lib/terminal';
import { isTraveler } from "@/store/cookie";
import TravelApp from "@/pages/TravelApp";
import Guide from "byte-guide"
import {IStep} from "byte-guide/dist/src/typings/guide"
import globalUserInfo from "../idp/global/userinfo"

type Props = {};
configure({ enforceActions: 'never' }) // 开启严格模式

let steps:IStep[] = []

const isModel = process.env.REACT_APP_VERSION === 'MODEL' || Boolean(process.env.NODE_OPEN)

const App: React.FC<Props> = () => {

  if(!isModel && !window.localStorage.getItem('hasGuide')){
    steps = [
      {
        selector: '#tour-side ul.ant-menu .workspace',
        placement:"right",
        title:"工作区",
        content:"这里提供了面向 AI 的集成开发环境，是算法开发、实验的核心工作区域"
      },
      {
        selector:"#tour-environment",
        placement:"top",
        title:"环境管理",
        content:"系统预装了基础 Python 环境，您可以在右侧包管理器中搜索、安装 Python 包，系统会持久化保存您安装的 Python 包。",
        offset:{
          y:140,
          x:0
        }
      },
      {
        selector:"#tour-file-management",
        placement:"right",
        title:"文件管理器",
        content:"在这里可以管理项目中的文件树，查看已连接的数据库、已挂载的第三方存储服务。"
      },
      {
        selector:"#notebook-tab-container",
        placement:"left",
        title:"Notebook",
        content:"您可以在 Notebook 交互式编程环境中，编写和运行 Python 代码，以构建、训练模型，还可以通过 SQL 查询和数据可视化快捷生成图表进行数据分析。"
      },
      {
        selector:"#tour-left-menu",
        placement:"right-top",
        title:"核心功能导航",
        content:
          <div>
            <div>左侧导航还提供核心功能模块，覆盖AI开发全生命周期。包括</div>
            <p>【数据集成】您可以在这里接入、管理数据源，包括主流数据库和各云厂商的对象存储服务。</p>
            <p>【数据标注】使用 Label Studio 提供的功能，进行多种数据类型的标注和探索，支持图像、视频、语音、文本、时间序列等类型数据的标注。</p>
            <p>【工作流】您可以在这里通过可视化拖拽方式，将数据处理、模型训练等任务串联成工作流，定时自动执行。</p>
            <p>【TensorBoard】您可以使用 TensorBoard 提供的可视化工具，对机器学习实验进行分析。</p>
            <p>【模型管理】模型资产是AI开发工作的沉淀和积累，您可以在这里进行模型资产的管理和共享。</p>
            <p>
              <a target={'_blank'} href="https://baihai-idp.yuque.com/mwvla8/doc">探索更多功能</a>
            </p>
          </div>
      },
    ]
  }else{
    steps = []
  }
  if(globalUserInfo.userInfo?.navType === 'AIGC'){
    steps = []
  }

  const [currentStep, setCurrentStep] = useState(0)
  const contentComponentRef = useRef({})


    const updateHeight = () => {
        Terminal.updateClientHeight(document.body.clientHeight);
        Terminal.setNext(1);
    }

    useEffect(() => {
        Terminal.updateClientHeight(document.body.clientHeight);
        Terminal.updateWorkspaceHeight(document.body.clientHeight);
        window.addEventListener("resize", updateHeight);
    }, []);

    // todo 如果是游客模式 渲染另外一个组件
    if (!Boolean(process.env.NODE_OPEN)) {
        if (isModel && isTraveler()) {
            return <TravelApp />
        }
    }

    return (
        <AliveScope>
            <Header />
            <Layout>
                <LeftNav />
                <Content ref={contentComponentRef}/>
            </Layout>
            <FooterBar />

            <Guide
              steps={steps}
              afterStepChange={(nextIndex, nextStep)=> {
                console.log('afterStepChange',nextIndex)
                if(nextIndex===1){
                  // @ts-ignore
                  contentComponentRef.current.setRightLineSelectKey("package")
                }else{
                    // @ts-ignore
                  contentComponentRef.current.setRightLineSelectKey("")
                }
              }}
              beforeStepChange={(stepIndex, step)=>{
                setCurrentStep(stepIndex+1)
              }}
              stepText={(stepIndex, stepCount) => {

                return `第${stepIndex+1}步，共6步`
              }}
              nextText="下一步"
              prevText="上一步"
              showPreviousBtn
              okText='完成引导'
              onClose={()=>{
                console.log('onClose')
                window.localStorage.setItem("hasGuide","true")
                // @ts-ignore
                contentComponentRef.current.setRightLineSelectKey("")
              }}
              // mask={currentStep!==1}
            />

        </AliveScope>
    )
}

export default App;
