import { useSelector, useDispatch } from "react-redux";
import { Layout } from "antd";
import { changeOperatorDecision, selectOperatorDecision } from "@/store/features/globalSlice";
import useShowFooter from "../../utils/hook/useShowFooter";
import { observer } from "mobx-react";
import { toJS } from 'mobx';
import globalData from "idp/global";

const { Footer } = Layout;

const FooterBar = (props) => {

    const isShowFooter = useShowFooter();
    const vis = useSelector(selectOperatorDecision);
    const dispatch = useDispatch();
    const footerBarMenuControl = globalData.footerBarMenuControl;
    if(!isShowFooter){
      return null
    }

    return (
        <Footer style={{ display: isShowFooter ? "flex" : "none" }} className="footbar" >
            <div className="footbar" onClick={(event) => { vis ? dispatch(changeOperatorDecision(false)) : null }} >
                <div className="runstate">
                    {
                        footerBarMenuControl.footerBarLeftMenuList.map(item => {
                            return item.component
                        })
                    }
                </div>
                <div className="totalstate">
                    {
                        footerBarMenuControl.footerBarRightMenuList.map((item) => {
                            return item.component
                        })
                    }
                </div>
            </div>
        </Footer>
    )
}

export default observer(FooterBar)
