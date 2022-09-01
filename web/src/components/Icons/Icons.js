import Icon from "@ant-design/icons"
import "antd/dist/antd.css"

const BHStop = () => (
  <svg
    width="1em"
    height="1em"
    fill="currentColor"
    aria-hidden="true"
    focusable="false"
    class=""
    viewBox="0 0 1024 1024"
  >
    <path d="M864 64H160C107 64 64 107 64 160v704c0 53 43 96 96 96h704c53 0 96-43 96-96V160c0-53-43-96-96-96z"></path>
  </svg>
)
const BHStopIcon = (props) => <Icon component={BHStop} {...props} />

const BHShare = () => (
  <svg width="1em" height="1em" viewBox="0 0 24 24" version="1.1">
    <g id="分享" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组" transform="translate(1.000000, 0.500000)">
        <circle id="椭圆形" fill="#EFEFEF" cx="18" cy="4" r="4"></circle>
        <circle id="椭圆形备份" fill="#EFEFEF" cx="18" cy="19" r="4"></circle>
        <circle id="椭圆形备份-2" fill="#EFEFEF" cx="4" cy="11" r="4"></circle>
        <polyline
          id="路径-11"
          stroke="#EFEFEF"
          strokeWidth="3"
          points="15.4262357 5.06761269 4.6830312 10.5801956 17.1303337 19"
        ></polyline>
      </g>
    </g>
  </svg>
)
const BHShareIcon = (props) => <Icon component={BHShare} {...props} />

//重启
const BHRestart = () => (
  <svg width="18px" height="20px" viewBox="0 0 20 18" version="1.1">
    <g id="重启" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组-2" transform="translate(-1.500000, 0.000000)">
        <rect
          id="矩形备份-3"
          fill="#FFFFFF"
          opacity="0"
          x="2.2408768"
          y="0"
          width="20"
          height="20"
        ></rect>
        <line
          x1="11.7408768"
          y1="6"
          x2="11.7408768"
          y2="9"
          id="路径-5"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
        ></line>
        <path
          d="M11.6977311,13.6568542 C11.9832967,13.6568542 12.2618731,13.6269297 12.5304955,13.5700453 C14.3399389,13.1868716 15.6977311,11.5804276 15.6977311,9.65685425 C15.6977311,7.44771525 13.9068701,5.65685425 11.6977311,5.65685425 C9.78711538,5.65685425 8.18937092,6.99641516 7.79244969,8.78758497 C7.7304329,9.06744553 7.69773105,9.35833093 7.69773105,9.65685425"
          id="椭圆形"
          stroke="currentColor"
          strokeWidth="2"
          transform="translate(11.697731, 9.656854) rotate(-225.000000) translate(-11.697731, -9.656854) "
        ></path>
        <g
          id="编组-7"
          transform="translate(11.619014, 10.000000) rotate(-12.000000) translate(-11.619014, -10.000000) translate(1.440877, 2.000000)"
        >
          <g
            id="编组备份"
            transform="translate(11.311414, 4.000000) scale(-1, -1) rotate(-180.000000) translate(-11.311414, -4.000000) translate(2.266553, 0.000000)"
          >
            <path
              d="M15,3.9989932 C14.9823047,3.96543198 13.4441957,-0.0764510589 7.63911503,0.00110189035 C3.21378615,0.293093922 0,3.9588977 0,8"
              id="椭圆形备份-3"
              stroke="currentColor"
              strokeWidth="2"
            ></path>
            <path
              d="M15.706031,2.94295019 L17.4910871,5.0958779 C17.5615888,5.18090881 17.5498105,5.30699286 17.4647795,5.3774945 C17.4292632,5.40694213 17.3846582,5.42320508 17.3385228,5.42352753 L13.7382144,5.44869104 C13.6277602,5.44946303 13.5375935,5.36054799 13.5368215,5.25009374 C13.5364911,5.20282388 13.5529146,5.15696486 13.5831785,5.1206517 L15.3984308,2.94256049 C15.4691476,2.85770849 15.5952611,2.8462497 15.6801131,2.91696657 C15.6895305,2.92481514 15.6982063,2.93351296 15.706031,2.94295019 Z"
              id="三角形备份"
              fill="currentColor"
              transform="translate(15.533791, 4.104720) rotate(138.000000) translate(-15.533791, -4.104720) "
            ></path>
          </g>
          <g
            id="编组"
            transform="translate(9.061702, 12.000000) scale(-1, -1) translate(-9.061702, -12.000000) translate(0.000000, 8.000000)"
          >
            <path
              d="M14.8823643,3.9989932 C14.8648077,3.96543198 13.3387612,-0.0764510589 7.57920617,0.00110189035 C2.87899171,0.244487569 0,3.9588977 0,8"
              id="椭圆形备份-3"
              stroke="currentColor"
              strokeWidth="2"
            ></path>
            <path
              d="M15.7427269,3.08941419 L17.5128018,5.22177179 C17.5833524,5.30676207 17.5716468,5.43285288 17.4866565,5.5034035 C17.4510415,5.5329676 17.4062746,5.54926326 17.3599885,5.5495121 L13.7968511,5.5686685 C13.6863958,5.56926234 13.5963726,5.48020198 13.5957788,5.36974663 C13.5955252,5.32258794 13.6119443,5.27685657 13.6421363,5.24062885 L15.4351989,3.08911486 C15.5059148,3.00426209 15.6320282,2.99280192 15.716881,3.06351786 C15.7262694,3.07134215 15.7349209,3.08001053 15.7427269,3.08941419 Z"
              id="三角形备份"
              fill="currentColor"
              transform="translate(15.574897, 4.237688) rotate(140.000000) translate(-15.574897, -4.237688) "
            ></path>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const BHRestartIcon = (props) => <Icon component={BHRestart} {...props} />

// 全部执行
const BHStartAll = () => (
  <svg width="18px" height="20px" viewBox="0 0 20 18" version="1.1">
    <g
      id="运行全部"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="编组-3">
        <g id="编组-2">
          <rect
            id="矩形备份"
            fill="#FFFFFF"
            opacity="0"
            x="0"
            y="0"
            width="20"
            height="20"
          ></rect>
          <circle
            id="椭圆形"
            stroke="currentColor"
            strokeWidth="2"
            cx="10"
            cy="10"
            r="8.5"
          ></circle>
          <polyline
            id="矩形备份-14"
            stroke="currentColor"
            transform="translate(8.571429, 10.071429) rotate(-45.000000) translate(-8.571429, -10.071429) "
            points="10.3897032 8.25315399 10.3897032 11.8897032 6.75315399 11.8897032"
          ></polyline>
        </g>
        <g id="编组" transform="translate(0.500000, 0.500000)">
          <polyline
            id="矩形"
            stroke="currentColor"
            transform="translate(11.571429, 9.571429) rotate(-45.000000) translate(-11.571429, -9.571429) "
            points="13.3897032 7.75315399 13.3897032 11.3897032 9.75315399 11.3897032"
          ></polyline>
          <polygon
            id="矩形"
            fill="currentColor"
            transform="translate(4.800000, 9.600000) rotate(-315.000000) translate(-4.800000, -9.600000) "
            points="2.67867966 7.47867966 6.92132034 7.47867966 6.92132034 11.7213203"
          ></polygon>
        </g>
      </g>
    </g>
  </svg>
)
const BHStartAllIcon = (props) => <Icon component={BHStartAll} {...props} />

//停止全部
const BHStopAll = () => (
  <svg width="18px" height="20px" viewBox="0 0 20 18" version="1.1">
    <g
      id="停止全部"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="编组-2">
        <rect
          id="矩形备份-2"
          fill="#FFFFFF"
          opacity="0"
          x="0"
          y="0"
          width="20"
          height="20"
        ></rect>
        <g id="编组" transform="translate(0.500000, 0.500000)">
          <rect
            id="矩形"
            fill="currentColor"
            x="6"
            y="6"
            width="7"
            height="7"
          ></rect>
          <circle
            id="椭圆形备份"
            stroke="currentColor"
            strokeWidth="2"
            cx="9.5"
            cy="9.5"
            r="8.5"
          ></circle>
        </g>
      </g>
    </g>
  </svg>
)
const BHStopAllIcon = (props) => <Icon component={BHStopAll} {...props} />

//开始
const BHStart = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <g
      id="搜索备份-5"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="编组">
        <rect
          id="矩形备份-2"
          fill="#FFFFFF"
          opacity="0"
          x="0"
          y="0"
          width="24"
          height="24"
        ></rect>
        <g id="编组-3" transform="translate(2.000000, 2.000000)">
          <circle
            id="椭圆形"
            fillOpacity="0"
            fill="#FFFFFF"
            cx="10"
            cy="10"
            r="10"
          ></circle>
          <circle
            id="椭圆形"
            stroke="#3495F0"
            strokeWidth="2"
            cx="10"
            cy="10"
            r="9"
          ></circle>
          <polygon
            id="三角形"
            fill="#3495F0"
            transform="translate(11.000000, 10.000000) rotate(-270.000000) translate(-11.000000, -10.000000) "
            points="11 5.5 16.5 14.5 5.5 14.5"
          ></polygon>
        </g>
      </g>
    </g>
  </svg>
)
const BHStartIcon = (props) => <Icon component={BHStart} {...props} />

// 文件夹
const BHFolder = () => (
  <svg width="1em" height="1em" viewBox="0 0 24 24" version="1.1">
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
    >
      <g id="文件" transform="translate(-12.000000, -74.000000)">
        <g id="1" transform="translate(12.000000, 74.000000)">
          <rect
            id="矩形"
            fillOpacity="0"
            fill="#FFFFFF"
            x="0"
            y="0"
            width="24"
            height="24"
          ></rect>
          <g id="编组-8备份" transform="translate(3.000000, 2.000000)">
            <rect
              id="矩形"
              stroke="currentColor"
              x="1.5"
              y="0.5"
              width="15"
              height="18"
              rx="2"
            ></rect>
            <g
              id="编组-20"
              transform="translate(0.000000, 3.000000)"
              fill="currentColor"
            >
              <rect id="矩形" x="0" y="0" width="3" height="1" rx="0.5"></rect>
              <rect
                id="矩形备份-42"
                x="0"
                y="8"
                width="3"
                height="1"
                rx="0.5"
              ></rect>
              <rect
                id="矩形备份-44"
                x="0"
                y="4"
                width="3"
                height="1"
                rx="0.5"
              ></rect>
              <rect
                id="矩形备份-43"
                x="0"
                y="12"
                width="3"
                height="1"
                rx="0.5"
              ></rect>
            </g>
            <path
              d="M5,0 L15,0 C16.1045695,-2.02906125e-16 17,0.8954305 17,2 L17,17 C17,18.1045695 16.1045695,19 15,19 L5,19 L5,19 L5,0 Z"
              id="矩形"
              fill="currentColor"
            ></path>
            <rect
              id="矩形"
              fill="#FFFFFF"
              x="7"
              y="4"
              width="6"
              height="1"
              rx="0.5"
            ></rect>
            <rect
              id="矩形备份-45"
              fill="#FFFFFF"
              x="7"
              y="7"
              width="6"
              height="1"
              rx="0.5"
            ></rect>
            <rect
              id="矩形备份-46"
              fill="#FFFFFF"
              x="7"
              y="10"
              width="4"
              height="1"
              rx="0.5"
            ></rect>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const BHFolderIcon = (props) => <Icon component={BHFolder} {...props} />

// 标题
const BHTitle = () => (
  <svg width="1em" height="1em" viewBox="0 0 31.5 31.5" version="1.1">
    <g id="笔记本" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g
        id="编组"
        transform="translate(4.000000, 5.000000)"
        fill="currentColor"
      >
        <rect id="矩形备份-4" x="5" y="10" width="19" height="2"></rect>
        <rect id="矩形备份-5" x="5" y="20" width="19" height="2"></rect>
        <rect id="矩形备份-6" x="5" y="0" width="19" height="2"></rect>
        <rect id="矩形备份" x="0" y="10" width="3" height="2"></rect>
        <rect id="矩形备份-2" x="0" y="20" width="3" height="2"></rect>
        <rect id="矩形备份-3" x="0" y="0" width="3" height="2"></rect>
      </g>
    </g>
  </svg>
)
const BHTitleIcon = (props) => <Icon component={BHTitle} {...props} />

// 备份
const BHDataset = () => (
  <svg width="1em" height="1em" viewBox="0 0 24 24" version="1.1">
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="文件" transform="translate(-12.000000, -131.000000)">
        <g id="2" transform="translate(12.000000, 131.000000)">
          <g
            id="编组-12"
            transform="translate(2.000000, 2.000000)"
            fill="currentColor"
          >
            <rect id="矩形" x="0" y="0" width="9" height="9" rx="2"></rect>
            <rect
              id="矩形备份-24"
              x="0"
              y="11"
              width="9"
              height="9"
              rx="2"
            ></rect>
            <rect
              id="矩形备份-23"
              x="11"
              y="0"
              width="9"
              height="9"
              rx="2"
            ></rect>
            <rect
              id="矩形备份-85"
              x="11"
              y="11"
              width="9"
              height="9"
              rx="2"
            ></rect>
          </g>
          <rect
            id="矩形"
            fillOpacity="0"
            fill="#FFFFFF"
            x="0"
            y="0"
            width="24"
            height="24"
          ></rect>
        </g>
      </g>
    </g>
  </svg>
)
const BHDatasetIcon = (props) => <Icon component={BHDataset} {...props} />

//package
const BHPackage = () => (
  <svg width="1em" height="1em" viewBox="0 0 32 32" version="1.1">
    <g id="3包" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组" transform="translate(-2.000000, 0.000000)">
        <rect
          id="矩形备份-6"
          fill="#EFEFEF"
          opacity="0"
          x="2"
          y="0"
          width="32"
          height="32"
        ></rect>
        <path
          d="M17.476568,3.25384116 L5.97561873,9.40172808 C5.87820616,9.45380043 5.84145065,9.57498189 5.893523,9.67239446 C5.91249649,9.70788845 5.9417372,9.73683239 5.9774229,9.75544281 L17.9514327,16 L17.9514327,16 L29.7540638,9.75489441 C29.8516957,9.70323458 29.8889635,9.58220967 29.8373037,9.48457771 C29.8188638,9.4497282 29.7905348,9.42110638 29.7558768,9.4023091 L18.4247519,3.25671143 C18.1292624,3.09644846 17.7730224,3.09537008 17.476568,3.25384116 Z"
          id="路径-3"
          stroke="currentColor"
          strokeWidth="2"
        ></path>
        <path
          d="M11.6746506,12.3801138 L0.288586358,19.8138305 C0.196096276,19.8742153 0.170069792,19.9981448 0.230454579,20.0906349 C0.24872999,20.118627 0.273730454,20.1415915 0.303170443,20.1574292 L11.8094368,26.347405 C12.1403141,26.5254056 12.5430441,26.5029196 12.8520497,26.2891918 L23.4396118,18.9661639 C23.530456,18.9033303 23.5531631,18.7787498 23.4903295,18.6879056 C23.4721539,18.6616276 23.4479471,18.6400866 23.4197357,18.6250864 L11.6746506,12.3801138 L11.6746506,12.3801138 Z"
          id="路径-3备份"
          stroke="currentColor"
          strokeWidth="2"
          transform="translate(11.858208, 19.508775) rotate(-298.000000) translate(-11.858208, -19.508775) "
        ></path>
        <path
          d="M23.677235,12.3937611 L12.266504,19.7918489 C12.173822,19.8519388 12.1474009,19.9757848 12.2074908,20.0684668 C12.2260472,20.0970881 12.2516156,20.1204877 12.2817652,20.1364414 L23.8696467,26.2681421 C24.1983672,26.4420838 24.5964623,26.4192334 24.9031212,26.2088214 L35.4396832,18.9792275 C35.530762,18.9167344 35.5539353,18.7922398 35.4914421,18.701161 C35.4732219,18.6746063 35.4488585,18.6528443 35.4204237,18.6377253 L23.677235,12.3937611 L23.677235,12.3937611 Z"
          id="路径-3备份-2"
          stroke="currentColor"
          strokeWidth="2"
          transform="translate(23.846200, 19.472414) scale(-1, 1) rotate(-298.000000) translate(-23.846200, -19.472414) "
        ></path>
      </g>
    </g>
  </svg>
)
const BHPackageIcon = (props) => <Icon component={BHPackage} {...props} />

//数学公式
// const BHFormula = () => (
//     <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" >
//     <g id="11数学格式" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
//         <g id="编组" transform="translate(-1.000000, 0.000000)">
//             <rect id="矩形备份-3" fill="#D8D8D8" opacity="0" x="1" y="0" width="16" height="16"></rect>
//             <image id="位图" x="0" y="1" width="14" height="14" xlink:href="data:image/png;base64,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"></image>
//             <text id="x" font-family="Helvetica" font-size="10" font-weight="normal" fill="currentColor">
//                 <tspan x="10.5" y="13">x</tspan>
//             </text>
//         </g>
//     </g>
// </svg>
// );
// const BHFormulaIcon = (props) => (
//     <Icon component={BHFormula} {...props} />
// );
//

// 新建文件夹
const BHAddFolder = () => (
  <svg width="16px" height="16px" viewBox="0 0 20 20" version="1.1">
    <g
      id="新建文件夹"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <path
        d="M14.0153184,18 L4,18 C3.44771525,18 3,17.5522847 3,17 L3,4 C3,3.44771525 3.44771525,3 4,3 L8.62927737,3 C8.76124043,3 8.88785265,3.05216848 8.98151151,3.14513226 L11.4025179,5.54817149 C11.4961767,5.64113528 11.6227889,5.69330375 11.754752,5.69330375 L18,5.69330375 C18.5522847,5.69330375 19,6.141019 19,6.69330375 L19,12.6 L19,12.6"
        id="路径"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinejoin="round"
      ></path>
      <line
        x1="13"
        y1="14.5"
        x2="19"
        y2="14.5"
        id="路径-2备份-2"
        stroke="currentColor"
        strokeWidth="1.4"
      ></line>
      <line
        x1="16"
        y1="11.4644827"
        x2="16"
        y2="17.4644827"
        id="路径-2备份-3"
        stroke="currentColor"
        strokeWidth="1.4"
      ></line>
    </g>
  </svg>
)
const BHAddFolderIcon = (props) => <Icon component={BHAddFolder} {...props} />

//搜索
const BHSearch = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <g id="搜索" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g
        id="编组"
        transform="translate(5.000000, 5.000000)"
        stroke="currentColor"
        strokeWidth="1.5"
      >
        <circle id="椭圆形" cx="6.5" cy="6.5" r="5.75"></circle>
        <line
          x1="10.764569"
          y1="10.7368293"
          x2="14.5"
          y2="14.5"
          id="路径-12"
        ></line>
      </g>
    </g>
  </svg>
)
const BHSearchIcon = (props) => <Icon component={BHSearch} {...props} />

//保存
const BHSave = () => (
  <svg width="17px" height="20px" viewBox="0 0 20 20" version="1.1">
    <g id="保存" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组">
        <rect
          id="矩形"
          fill="currentColor"
          opacity="0"
          x="0"
          y="0"
          width="20"
          height="20"
        ></rect>
        <path
          d="M12.2149139,2 C12.4788188,2 12.7320247,2.10432023 12.9193387,2.29022141 L12.9193387,2.29022141 L15.3521288,4.70459001 L17.6972257,6.98562524 C17.8907964,7.17389633 18,7.43244799 18,7.70247687 L18,7.70247687 L18,18 L2,18 L2,2 Z"
          id="矩形"
          stroke="currentColor"
          strokeWidth="2"
        ></path>
        <path
          d="M5.79287391,2.89108586 L5.79287391,8.07529639 C5.79287391,8.35143876 6.01673154,8.57529639 6.29287391,8.57529639 L9.97368421,8.57529639 C10.2498266,8.57529639 10.4736842,8.35143876 10.4736842,8.07529639 L10.4736842,2.89108586 L10.4736842,2.89108586 L5.79287391,2.89108586 Z"
          id="路径-8"
          stroke="currentColor"
          strokeWidth="2"
        ></path>
      </g>
    </g>
  </svg>
)
const BHSaveIcon = (props) => <Icon component={BHSave} {...props} />

//新建文件
const BHAddFile = () => (
  <svg
    width="16px"
    height="16px"
    viewBox="0 0 20 20"
    version="1.1"
    fill="currentColor"
  >
    <g
      id="新建文件"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g
        id="编组"
        transform="translate(3.000000, 1.000000)"
        stroke="currentColor"
        strokeWidth="2"
      >
        <path
          d="M0.991679268,1.0083249 L9.6609307,1.08046191 L12.2283144,3.09561441 L15,5.14685467 L15,17 L1,17 L0.991679268,1.0083249 Z"
          id="矩形"
        ></path>
      </g>
      <line
        x1="8"
        y1="12.5"
        x2="14"
        y2="12.5"
        id="路径-2"
        stroke="currentColor"
        strokeWidth="1.4"
      ></line>
      <line
        x1="11"
        y1="9.46448272"
        x2="11"
        y2="15.4644827"
        id="路径-2备份"
        stroke="currentColor"
        strokeWidth="1.4"
      ></line>
    </g>
  </svg>
)
const BHAddFileIcon = (props) => <Icon component={BHAddFile} {...props} />

//删除
const BHDelete = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g id="4删除" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <rect
        id="矩形"
        fill="currentColor"
        x="2"
        y="2"
        width="12"
        height="2"
        rx="1"
      ></rect>
      <path
        d="M13,4 L12.2719109,14.0720996 C12.2341084,14.5950447 11.7988231,15 11.2745134,15 L4.74479474,15 C4.22121873,15 3.78628177,14.5961415 3.74753658,14.0740011 L3,4 L13,4 Z M6.65,5 C6.29101491,5 6,5.29101491 6,5.65 L6,5.65 L6,11.35 C6,11.7089851 6.29101491,12 6.65,12 C7.00898509,12 7.3,11.7089851 7.3,11.35 L7.3,11.35 L7.3,5.65 C7.3,5.29101491 7.00898509,5 6.65,5 Z M9.65,5 C9.29101491,5 9,5.29101491 9,5.65 L9,5.65 L9,11.35 C9,11.7089851 9.29101491,12 9.65,12 C10.0089851,12 10.3,11.7089851 10.3,11.35 L10.3,11.35 L10.3,5.65 C10.3,5.29101491 10.0089851,5 9.65,5 Z"
        id="形状结合"
        fill="currentColor"
      ></path>
      <circle id="椭圆形" fill="currentColor" cx="8" cy="2" r="1.5"></circle>
    </g>
  </svg>
)
const BHDeleteIcon = (props) => <Icon component={BHDelete} {...props} />
//更多
const BHMore = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g id="5更多" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组">
        <rect
          id="矩形备份-4"
          fill="currentColor"
          opacity="0"
          x="0"
          y="0"
          width="16"
          height="16"
        ></rect>
        <circle id="椭圆形" fill="currentColor" cx="8" cy="3" r="1.5"></circle>
        <circle
          id="椭圆形备份"
          fill="currentColor"
          cx="8"
          cy="8"
          r="1.5"
        ></circle>
        <circle
          id="椭圆形备份-2"
          fill="currentColor"
          cx="8"
          cy="13"
          r="1.5"
        ></circle>
      </g>
    </g>
  </svg>
)
const BHMoreIcon = (props) => <Icon component={BHMore} {...props} />
//清除
const BHClean = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g id="3清除" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g
        id="编组-8"
        transform="translate(1.758801, 1.000000)"
        fill="currentColor"
      >
        <rect id="矩形" x="0.241198923" y="2" width="12" height="4"></rect>
        <rect
          id="矩形备份"
          transform="translate(6.241199, 3.000000) rotate(-270.000000) translate(-6.241199, -3.000000) "
          x="3.24119892"
          y="0.5"
          width="6"
          height="5"
        ></rect>
        <path
          d="M12.2411989,7 L12.2411989,13.0659976 L12.5097272,14 L10.241,14 L10.2411989,11 L8.24119892,11 L8.241,14 L7.241,14 L7.24119892,11 L5.24119892,11 L5.241,14 L4.241,14 L4.24119892,11 L2.24119892,11 L2.241,14 L0,14 L0.241198923,13.0216769 L0.241198923,7 L12.2411989,7 Z"
          id="形状结合"
        ></path>
      </g>
    </g>
  </svg>
)
const BHCleanIcon = (props) => <Icon component={BHClean} {...props} />

//上箭头（填充）
const BHArrowUp = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g
      id="1上箭头"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <path
        d="M8.81602448,3.15203456 L15.3822377,12.4219827 C15.7014679,12.8726605 15.5949085,13.4967943 15.1442306,13.8160245 C14.9752536,13.9357165 14.7732867,14 14.5662132,14 L1.43378676,14 C0.881502014,14 0.433786764,13.5522847 0.433786764,13 C0.433786764,12.7929265 0.498070224,12.5909597 0.617762283,12.4219827 L7.18397552,3.15203456 C7.50320568,2.70135669 8.12733946,2.59479726 8.57801734,2.91402742 C8.67023101,2.97934544 8.75070646,3.05982089 8.81602448,3.15203456 Z"
        id="三角形"
        fill="currentColor"
      ></path>
    </g>
  </svg>
)
const BHArrowUpIcon = (props) => <Icon component={BHArrowUp} {...props} />
//下箭头（填充）
const BHArrowDown = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g
      id="2下箭头"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <path
        d="M8.81602448,3.15203456 L15.3822377,12.4219827 C15.7014679,12.8726605 15.5949085,13.4967943 15.1442306,13.8160245 C14.9752536,13.9357165 14.7732867,14 14.5662132,14 L1.43378676,14 C0.881502014,14 0.433786764,13.5522847 0.433786764,13 C0.433786764,12.7929265 0.498070224,12.5909597 0.617762283,12.4219827 L7.18397552,3.15203456 C7.50320568,2.70135669 8.12733946,2.59479726 8.57801734,2.91402742 C8.67023101,2.97934544 8.75070646,3.05982089 8.81602448,3.15203456 Z"
        id="三角形备份"
        fill="currentColor"
        transform="translate(8.000000, 8.000000) rotate(-180.000000) translate(-8.000000, -8.000000) "
      ></path>
    </g>
  </svg>
)
const BHArrowDownIcon = (props) => <Icon component={BHArrowDown} {...props} />

//添加图片(For markdown)
const BHAddIMG = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g
      id="6添加图片"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="编组-3">
        <g id="编组-2"></g>
        <g id="编组-4" transform="translate(1.000000, 2.000000)">
          <rect
            id="矩形"
            stroke="#666666"
            x="0.5"
            y="0.5"
            width="13"
            height="11"
          ></rect>
          <path
            d="M0.988355971,11.1095684 C1.19042693,10.8471433 2.36097494,9.70436424 4.5,7.68123134 L7,10.6035774 L10.7807637,5.35361785 L13.5219071,9.27214554"
            id="路径-4"
            stroke="#666666"
          ></path>
          <circle id="椭圆形" fill="#666666" cx="3.5" cy="3.5" r="1.5"></circle>
        </g>
      </g>
    </g>
  </svg>
)
const BHAddIMGIcon = (props) => <Icon component={BHAddIMG} {...props} />

//展开全部(For topToolsBar)
const BHExtendAll = (e) => (
  <svg width="20px" height="20px" viewBox="0 0 20 20" version="1.1">
    <g
      id="6一键展开备份-2"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="编组-4" transform="translate(-0.000000, 0.000000)">
        <rect
          id="矩形备份-4"
          fill="#FFFFFF"
          opacity="0"
          x="0"
          y="0"
          width="20"
          height="20"
        ></rect>
        <g
          id="编组-3"
          transform="translate(2.000000, 2.000000)"
          stroke="currentColor"
          strokeLinecap="round"
          strokeWidth="2"
        >
          <g id="编组">
            <polyline
              id="路径"
              points="11.3060241 0 11.3060241 11.2763819 0 11.2763819"
            ></polyline>
            <polyline
              id="路径备份"
              points="15 3.72361809 15 15 3.6939759 15"
            ></polyline>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const BHExtendAllIcon = (props) => <Icon component={BHExtendAll} {...props} />

// 上传
const BHUpload = (e) => (
  <svg
    width="16px"
    height="16px"
    viewBox="0 0 20 20"
    version="1.1"
    fill="currentColor"
  >
    <g
      id="上传备份-3"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
      strokeLinejoin="round"
    >
      <polygon
        id="Path"
        stroke="currentColor"
        strokeWidth="0.6"
        fill="currentColor"
        fillRule="nonzero"
        points="3.96428571 11.2857143 4.84553571 12.1921429 9.58928571 7.31928571 9.58928571 19 10.8392857 19 10.8392857 7.31928571 15.5830357 12.1921429 16.4642857 11.2857143 10.2142857 4.85714286"
      ></polygon>
      <path
        d="M3.96428571,4.85714286 L3.96428571,2.28571429 L16.4642857,2.28571429 L16.4642857,4.85714286 L17.7142857,4.85714286 L17.7142857,2.28571429 C17.7142857,1.57563389 17.1546417,1 16.4642857,1 L3.96428571,1 C3.27392978,1 2.71428571,1.57563389 2.71428571,2.28571429 L2.71428571,4.85714286 L3.96428571,4.85714286 Z"
        id="Path"
        stroke="currentColor"
        strokeWidth="0.6"
        fill="currentColor"
        fillRule="nonzero"
      ></path>
    </g>
  </svg>
)
const BHUploadIcon = (props) => <Icon component={BHUpload} {...props} />

// 上传文件
const BHUploadFile = () => (
  <svg width="16px" height="16px" viewBox="0 0 20 20" version="1.1">
    <g
      id="新建文件备份"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g
        id="编组"
        transform="translate(2.000000, 1.000000)"
        stroke="currentColor"
        strokeWidth="2"
      >
        <path
          d="M0.991679268,1.0083249 L9.6609307,1.08046191 L12.2283144,3.09561441 L15,5.14685467 L15,17 L1,17 L0.991679268,1.0083249 Z"
          id="矩形"
        ></path>
      </g>
      <polygon
        id="Path"
        stroke="currentColor"
        strokeWidth="0.6"
        fill="currentColor"
        fillRule="nonzero"
        strokeLinejoin="round"
        points="7.8525 12.1118182 9.75 10.0445455 9.75 15 10.25 15 10.25 10.0445455 12.1475 12.1118182 12.5 11.7272727 10 9 7.5 11.7272727"
      ></polygon>
    </g>
  </svg>
)
const BHUploadFileIcon = (props) => <Icon component={BHUploadFile} {...props} />

// 上传文件夹
const BHUploadFolder = () => (
  <svg width="16px" height="16px" viewBox="0 0 20 20" version="1.1">
    <g
      id="新建文件夹备份"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
      strokeLinejoin="round"
    >
      <path
        d="M13.3922332,18 L2,18 C1.44771525,18 1,17.5522847 1,17 L1,3 C1,2.44771525 1.44771525,2 2,2 L7.36642724,2 C7.49378861,2 7.61634951,2.04860242 7.70909706,2.13588822 L10.472936,4.73696912 C10.5656835,4.82425492 10.6882444,4.87285734 10.8156058,4.87285734 L18,4.87285734 C18.5522847,4.87285734 19,5.32057259 19,5.87285734 L19,12.5194088 L19,12.5194088 L19,18 L13.3922332,18 Z"
        id="路径"
        stroke="currentColor"
        strokeWidth="2"
      ></path>
      <polygon
        id="Path备份"
        stroke="currentColor"
        strokeWidth="0.6"
        fill="currentColor"
        fillRule="nonzero"
        points="7.8525 12.1118182 9.75 10.0445455 9.75 15 10.25 15 10.25 10.0445455 12.1475 12.1118182 12.5 11.7272727 10 9 7.5 11.7272727"
      ></polygon>
    </g>
  </svg>
)
const BHUploadFolderIcon = (props) => (
  <Icon component={BHUploadFolder} {...props} />
)

// 刷新
const BHRefresh = () => (
  <svg
    width="16px"
    height="16px"
    viewBox="0 0 20 20"
    version="1.1"
    fill="currentColor"
  >
    <g
      id="refresh"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
      strokeLinecap="round"
    >
      <g
        id="编组-6"
        transform="translate(9.000000, 10.500000) scale(-1, 1) translate(-9.000000, -10.500000) translate(-1.000000, 0.500000)"
        stroke="currentColor"
        strokeWidth="2"
      >
        <path
          d="M10,17.1241703 C13.2621286,17.1241703 16.0121667,14.9316498 16.8568799,11.9398431 C17.0310057,11.3231242 17.1241703,10.672442 17.1241703,10 C17.1241703,9.24145626 17.00562,8.51060143 16.7860364,7.82495264 C15.866639,4.95413732 13.1760269,2.8758297 10,2.8758297 C6.06542939,2.8758297 2.8758297,6.06542939 2.8758297,10 C2.8758297,12.4779432 4.14092773,14.6604069 6.06047019,15.9367375"
          id="椭圆形"
          transform="translate(10.000000, 10.000000) rotate(52.000000) translate(-10.000000, -10.000000) "
        ></path>
        <path
          d="M0.361416657,8.38663691 L2.66369767,10.6889179 C2.78085496,10.8060752 2.97080445,10.8060752 3.08796174,10.6889179 L5.39024275,8.38663691 L5.39024275,8.38663691"
          id="路径-10"
          transform="translate(2.875830, 9.643843) rotate(7.000000) translate(-2.875830, -9.643843) "
        ></path>
      </g>
    </g>
  </svg>
)
const BHRefreshIcon = (props) => <Icon component={BHRefresh} {...props} />

// cell 未执行
const BHCellReady = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <g id="运行" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组">
        <rect
          id="矩形备份-2"
          fill="#FFFFFF"
          opacity="0"
          x="0"
          y="0"
          width="24"
          height="24"
        ></rect>
        <g id="编组-3" transform="translate(0.500000, 0.500000)">
          <circle
            id="椭圆形"
            stroke="#3495F0"
            strokeWidth="2"
            cx="11.5"
            cy="11.5"
            r="10.5"
          ></circle>
          <polygon
            id="三角形"
            fill="#3495F0"
            transform="translate(12.500000, 11.500000) rotate(-270.000000) translate(-12.500000, -11.500000) "
            points="12.5 7 18 16 7 16"
          ></polygon>
        </g>
      </g>
    </g>
  </svg>
)
const BHCellReadyIcon = (props) => <Icon component={BHCellReady} {...props} />

// cell 等待执行
const BHCellPending = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <defs>
      <filter colorInterpolationFilters="auto" id="filter-1">
        <feColorMatrix
          in="SourceGraphic"
          type="matrix"
          values="0 0 0 0 0.600000 0 0 0 0 0.600000 0 0 0 0 0.600000 0 0 0 1.000000 0"
        />
      </filter>
      <circle id="path-2" cx="12" cy="12" r="11.5">
        <animateTransform
          attributeName="transform"
          attributeType="XML"
          type="rotate"
          from="0 12 12"
          to="360 12 12"
          dur="1.5s"
          repeatCount="indefinite"
        />
      </circle>

      <mask
        id="mask-3"
        maskContentUnits="userSpaceOnUse"
        maskUnits="objectBoundingBox"
        x="0"
        y="0"
        width="23"
        height="23"
        fill="white"
      >
        <use xlinkHref="#path-2" />
      </mask>
    </defs>
    <g
      id="运行备份-3"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g filter="url(#filter-1)" id="编组-2">
        <g transform="translate(9.000000, 6.500000)" id="编组-3" fill="#3495F0">
          <polygon
            id="三角形"
            transform="translate(4.000000, 5.500000) rotate(-270.000000) translate(-4.000000, -5.500000) "
            points="4 1.5 9.5 9.5 -1.5 9.5"
          />
        </g>
      </g>
      <use
        id="椭圆形备份-2"
        stroke="#979797"
        mask="url(#mask-3)"
        strokeWidth="4"
        strokeDasharray="4,2"
        xlinkHref="#path-2"
      />
    </g>
  </svg>
)
const BHCellPendingIcon = (props) => (
  <Icon component={BHCellPending} {...props} />
)

// cell 执行中
const BHCellExecuting = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <defs>
      <circle id="path-1" cx="12" cy="12" r="11.5">
        <animateTransform
          attributeName="transform"
          attributeType="XML"
          type="rotate"
          from="0 12 12"
          to="360 12 12"
          dur="1.5s"
          repeatCount="indefinite"
        />
      </circle>
      <mask
        id="mask-2"
        maskContentUnits="userSpaceOnUse"
        maskUnits="objectBoundingBox"
        x="0"
        y="0"
        width="23"
        height="23"
        fill="white"
      >
        <use xlinkHref="#path-1"></use>
      </mask>
    </defs>
    <g
      id="运行备份-4"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <rect id="矩形" fill="#999999" x="8" y="8" width="8" height="8"></rect>
      <use
        id="椭圆形备份-3"
        stroke="#979797"
        mask="url(#mask-2)"
        strokeWidth="4"
        strokeDasharray="4,2"
        xlinkHref="#path-1"
      ></use>
    </g>
  </svg>
)
const BHCellExecutingIcon = (props) => (
  <Icon component={BHCellExecuting} {...props} />
)

// 联系客户
const BHContactUs = () => (
  <svg width="32px" height="32px" viewBox="0 0 32 32" version="1.1">
    <g
      id="客服备份"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <rect
        id="矩形"
        stroke="#FFFFFF"
        strokeWidth="1.5"
        x="5.5"
        y="13.75"
        width="5.5"
        height="8.5"
        rx="2.75"
      ></rect>
      <rect
        id="矩形备份"
        stroke="#FFFFFF"
        strokeWidth="1.5"
        x="21"
        y="13.75"
        width="5.5"
        height="8.5"
        rx="2.75"
      ></rect>
      <path
        d="M24.25,22 C24.25,24.7614237 22.0114237,27 19.25,27 L17.25,27 L17.25,27"
        id="矩形"
        stroke="#FFFFFF"
        strokeWidth="1.5"
      ></path>
      <path
        d="M26.5,19.5 L26.5,23.5 C26.5,29.2989899 21.7989899,34 16,34 C10.2010101,34 5.5,29.2989899 5.5,23.5 L5.5,19.5 L5.5,19.5"
        id="矩形"
        stroke="#FFFFFF"
        strokeWidth="1.5"
        transform="translate(16.000000, 19.500000) rotate(-180.000000) translate(-16.000000, -19.500000) "
      ></path>
      <line
        x1="16.75"
        y1="25"
        x2="16.75"
        y2="27.75"
        id="路径-9"
        stroke="#FFFFFF"
        strokeWidth="1.5"
      ></line>
    </g>
  </svg>
)
const BHContactUsIcon = (props) => <Icon component={BHContactUs} {...props} />

// 终端
const BHTerminal = () => (
  <svg width="1em" height="1em" viewBox="0 0 24 24" version="1.1">
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="文件" transform="translate(-14.000000, -229.000000)">
        <g id="4" transform="translate(14.000000, 229.000000)">
          <polyline
            id="矩形"
            stroke="currentColor"
            strokeWidth="2"
            transform="translate(5.500000, 13.500000) rotate(-45.000000) translate(-5.500000, -13.500000) "
            points="8 11 8 16 3 16"
          ></polyline>
          <path
            d="M20,3 C21.1045695,3 22,3.8954305 22,5 L22,19 C22,20.1045695 21.1045695,21 20,21 L4,21 C2.8954305,21 2,20.1045695 2,19 L2,5 C2,3.8954305 2.8954305,3 4,3 L20,3 Z M20,8 L4,8 L4,19 L20,19 L20,8 Z"
            id="形状结合"
            fill="currentColor"
          ></path>
          <g id="编组-19备份"></g>
          <rect
            id="矩形"
            fill="currentColor"
            x="12"
            y="16"
            width="7"
            height="2"
          ></rect>
        </g>
      </g>
    </g>
  </svg>
)
const BHTerminalIcon = (props) => <Icon component={BHTerminal} {...props} />
//DAG
const BHDAG = () => (
  <svg width="1em" height="1em" viewBox="0 0 32 32" version="1.1">
    <rect
      stroke="currentColor"
      fill="none"
      strokeWidth="2"
      x="2"
      y="6"
      width="27"
      height="20"
      rx="2"
    ></rect>
    <text x="24" y="64" transform="scale(0.3)" fill="currentColor">
      DAG
    </text>
  </svg>
)
const BHDAGIcon = (props) => <Icon component={BHDAG} {...props} />

const BHPipeLine = () => (
  <svg width="1em" height="1em" viewBox="0 0 24 24" version="1.1">
    <defs>
      <filter colorInterpolationFilters="auto" id="filter-1">
        <feColorMatrix
          in="SourceGraphic"
          type="matrix"
          // values="currentColor"
        ></feColorMatrix>
      </filter>
    </defs>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="文件" transform="translate(-12.000000, -183.000000)">
        <g id="3" transform="translate(12.000000, 183.000000)">
          <rect
            id="矩形"
            fill="currentColor"
            opacity="0"
            x="0"
            y="0"
            width="24"
            height="24"
          ></rect>
          <g filter="url(#filter-1)" id="工作流备份">
            <g transform="translate(2.085938, 2.500000)">
              <path
                d="M14.2900983,0.5 L5.51786989,0.5 C5.11675007,0.5 4.79222599,0.823762708 4.79222599,1.22394146 L4.79222599,4.44347 C4.79222599,4.84364875 5.11675007,5.16741146 5.51786989,5.16741146 L14.2900983,5.16741146 C14.6912182,5.16741146 15.0157422,4.84364875 15.0157422,4.44347 L15.0157422,1.22394146 C15.0157422,0.823762708 14.6912182,0.5 14.2900983,0.5 L14.2900983,0.5 Z M13.5644544,3.71952854 L6.24351379,3.71952854 L6.24351379,1.94788292 L13.5644544,1.94788292 L13.5644544,3.71952854 L13.5644544,3.71952854 Z M12.6775563,7.15825047 L7.13041188,7.15825047 C5.84037829,7.15825047 4.79222599,8.20595464 4.79222599,9.49095073 C4.79222599,10.7779578 5.84239397,11.823651 7.13041188,11.823651 L12.6775563,11.823651 C13.9675899,11.823651 15.0157422,10.7759468 15.0157422,9.49095073 C15.0157422,8.20394369 13.9675899,7.15825047 12.6775563,7.15825047 L12.6775563,7.15825047 Z M12.6775563,10.3757681 L7.13041188,10.3757681 C6.64060225,10.3757681 6.24351379,9.97760027 6.24351379,9.49095073 C6.24351379,9.00229025 6.64261793,8.60613339 7.13041188,8.60613339 L12.6775563,8.60613339 C13.167366,8.60613339 13.5644544,9.0043012 13.5644544,9.49095073 C13.5644544,9.97961122 13.167366,10.3757681 12.6775563,10.3757681 L12.6775563,10.3757681 Z M14.2578475,13.8325885 L5.48360337,13.8325885 C5.08248355,13.8325885 4.75795948,14.1563512 4.75795948,14.55653 L4.75795948,17.7760585 C4.75795948,18.1762373 5.08248355,18.5 5.48360337,18.5 L14.2578475,18.5 C14.6589673,18.5 14.9834914,18.1762373 14.9834914,17.7760585 L14.9834914,14.55653 C14.9834914,14.1563512 14.6569516,13.8325885 14.2578475,13.8325885 Z M13.5322036,17.0521171 L6.20924727,17.0521171 L6.20924727,15.2804715 L13.5322036,15.2804715 L13.5322036,17.0521171 Z M3.2603111,2.56725506 C2.56691804,2.91716009 1.98035589,3.45207239 1.56714201,4.10965255 C1.13981838,4.7873422 0.9140625,5.57161211 0.9140625,6.37398056 C0.9140625,7.33119205 1.22447683,8.23410792 1.81103898,8.98821361 C1.81305466,8.99022456 1.81507034,8.99424645 1.81708602,8.9962574 L0.974532825,9.53318065 C0.883827338,9.59149816 0.908015468,9.7302536 1.0128307,9.75237404 L3.34900091,10.2651659 C3.41350259,10.2792425 3.47800427,10.2390236 3.49211401,10.1746732 L4.01619016,7.85001676 C4.04037829,7.74544744 3.92346899,7.6650095 3.83276351,7.72131605 L3.03455522,8.22606413 C3.01238277,8.18182326 2.98617896,8.13959334 2.9559438,8.10138532 C2.56893372,7.60468104 2.36333462,7.00742934 2.36333462,6.37398056 C2.36333462,5.30616691 2.95795948,4.34292258 3.91339061,3.85828399 C4.27016552,3.67729863 4.41327863,3.24293375 4.23186765,2.88498492 C4.05045668,2.52703609 3.61708602,2.38626969 3.2603111,2.56725506 L3.2603111,2.56725506 Z M18.6822596,16.04061 L17.9505687,15.4393364 C17.9707254,15.4192269 17.9928979,15.3971065 18.011039,15.3729751 C18.5935698,14.6530555 18.9140625,13.7481287 18.9140625,12.8210814 C18.9140625,12.095129 18.7185418,11.3812423 18.3516885,10.7578483 C17.9949136,10.1545637 17.4849472,9.64981566 16.8762126,9.29789968 C16.773413,9.23958217 16.6685978,9.18327561 16.5617669,9.1330019 C16.1989449,8.96408223 15.7675899,9.12093621 15.5962573,9.48290694 C15.4269404,9.84487767 15.5841633,10.2752206 15.9469852,10.4461513 C16.0155183,10.4783264 16.0840513,10.5145235 16.1505687,10.5507206 C16.9588553,11.0232935 17.4627747,11.8920232 17.4627747,12.8230924 C17.4627747,13.4283879 17.261207,13.9954754 16.8822596,14.4660373 C16.8661342,14.4861468 16.8520244,14.5062563 16.8379147,14.5263658 L16.0659102,13.892917 C15.9832674,13.8245447 15.8582954,13.890906 15.8683738,13.9974863 L16.1062238,16.3724165 C16.1122708,16.4387778 16.1727411,16.4870406 16.2372428,16.4810077 L18.6137266,16.2537705 C18.7245888,16.2437158 18.7649024,16.1089822 18.6822596,16.04061 Z"
                id="形状"
                fill="currentColor"
                fillRule="nonzero"
              ></path>
            </g>
          </g>
        </g>
      </g>
    </g>
  </svg>
)

const BHPipeLineIcon = (props) => <Icon component={BHPipeLine} {...props} />

const BHPipeLineSelected = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <defs>
      <filter colorInterpolationFilters="auto" id="filter-1">
        <feColorMatrix
          in="SourceGraphic"
          type="matrix"
          values="0 0 0 0 0.215686 0 0 0 0 0.576471 0 0 0 0 0.937255 0 0 0 1.000000 0"
        ></feColorMatrix>
      </filter>
    </defs>
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="文件" transform="translate(-8.000000, -607.000000)">
        <g id="编组-9" transform="translate(8.000000, 607.000000)">
          <rect
            id="矩形备份-6"
            fill="#D8D8D8"
            opacity="0"
            x="0"
            y="0"
            width="24"
            height="24"
          ></rect>
          <g filter="url(#filter-1)" id="工作流备份">
            <g transform="translate(2.085938, 2.500000)">
              <path
                d="M14.2900983,0.5 L5.51786989,0.5 C5.11675007,0.5 4.79222599,0.823762708 4.79222599,1.22394146 L4.79222599,4.44347 C4.79222599,4.84364875 5.11675007,5.16741146 5.51786989,5.16741146 L14.2900983,5.16741146 C14.6912182,5.16741146 15.0157422,4.84364875 15.0157422,4.44347 L15.0157422,1.22394146 C15.0157422,0.823762708 14.6912182,0.5 14.2900983,0.5 L14.2900983,0.5 Z M13.5644544,3.71952854 L6.24351379,3.71952854 L6.24351379,1.94788292 L13.5644544,1.94788292 L13.5644544,3.71952854 L13.5644544,3.71952854 Z M12.6775563,7.15825047 L7.13041188,7.15825047 C5.84037829,7.15825047 4.79222599,8.20595464 4.79222599,9.49095073 C4.79222599,10.7779578 5.84239397,11.823651 7.13041188,11.823651 L12.6775563,11.823651 C13.9675899,11.823651 15.0157422,10.7759468 15.0157422,9.49095073 C15.0157422,8.20394369 13.9675899,7.15825047 12.6775563,7.15825047 L12.6775563,7.15825047 Z M12.6775563,10.3757681 L7.13041188,10.3757681 C6.64060225,10.3757681 6.24351379,9.97760027 6.24351379,9.49095073 C6.24351379,9.00229025 6.64261793,8.60613339 7.13041188,8.60613339 L12.6775563,8.60613339 C13.167366,8.60613339 13.5644544,9.0043012 13.5644544,9.49095073 C13.5644544,9.97961122 13.167366,10.3757681 12.6775563,10.3757681 L12.6775563,10.3757681 Z M14.2578475,13.8325885 L5.48360337,13.8325885 C5.08248355,13.8325885 4.75795948,14.1563512 4.75795948,14.55653 L4.75795948,17.7760585 C4.75795948,18.1762373 5.08248355,18.5 5.48360337,18.5 L14.2578475,18.5 C14.6589673,18.5 14.9834914,18.1762373 14.9834914,17.7760585 L14.9834914,14.55653 C14.9834914,14.1563512 14.6569516,13.8325885 14.2578475,13.8325885 Z M13.5322036,17.0521171 L6.20924727,17.0521171 L6.20924727,15.2804715 L13.5322036,15.2804715 L13.5322036,17.0521171 Z M3.2603111,2.56725506 C2.56691804,2.91716009 1.98035589,3.45207239 1.56714201,4.10965255 C1.13981838,4.7873422 0.9140625,5.57161211 0.9140625,6.37398056 C0.9140625,7.33119205 1.22447683,8.23410792 1.81103898,8.98821361 C1.81305466,8.99022456 1.81507034,8.99424645 1.81708602,8.9962574 L0.974532825,9.53318065 C0.883827338,9.59149816 0.908015468,9.7302536 1.0128307,9.75237404 L3.34900091,10.2651659 C3.41350259,10.2792425 3.47800427,10.2390236 3.49211401,10.1746732 L4.01619016,7.85001676 C4.04037829,7.74544744 3.92346899,7.6650095 3.83276351,7.72131605 L3.03455522,8.22606413 C3.01238277,8.18182326 2.98617896,8.13959334 2.9559438,8.10138532 C2.56893372,7.60468104 2.36333462,7.00742934 2.36333462,6.37398056 C2.36333462,5.30616691 2.95795948,4.34292258 3.91339061,3.85828399 C4.27016552,3.67729863 4.41327863,3.24293375 4.23186765,2.88498492 C4.05045668,2.52703609 3.61708602,2.38626969 3.2603111,2.56725506 L3.2603111,2.56725506 Z M18.6822596,16.04061 L17.9505687,15.4393364 C17.9707254,15.4192269 17.9928979,15.3971065 18.011039,15.3729751 C18.5935698,14.6530555 18.9140625,13.7481287 18.9140625,12.8210814 C18.9140625,12.095129 18.7185418,11.3812423 18.3516885,10.7578483 C17.9949136,10.1545637 17.4849472,9.64981566 16.8762126,9.29789968 C16.773413,9.23958217 16.6685978,9.18327561 16.5617669,9.1330019 C16.1989449,8.96408223 15.7675899,9.12093621 15.5962573,9.48290694 C15.4269404,9.84487767 15.5841633,10.2752206 15.9469852,10.4461513 C16.0155183,10.4783264 16.0840513,10.5145235 16.1505687,10.5507206 C16.9588553,11.0232935 17.4627747,11.8920232 17.4627747,12.8230924 C17.4627747,13.4283879 17.261207,13.9954754 16.8822596,14.4660373 C16.8661342,14.4861468 16.8520244,14.5062563 16.8379147,14.5263658 L16.0659102,13.892917 C15.9832674,13.8245447 15.8582954,13.890906 15.8683738,13.9974863 L16.1062238,16.3724165 C16.1122708,16.4387778 16.1727411,16.4870406 16.2372428,16.4810077 L18.6137266,16.2537705 C18.7245888,16.2437158 18.7649024,16.1089822 18.6822596,16.04061 Z"
                id="形状"
                fill="#000000"
                fillRule="nonzero"
              ></path>
            </g>
          </g>
        </g>
      </g>
    </g>
  </svg>
)

const BHPipeLineSelectedIcon = (props) => (
  <Icon component={BHPipeLineSelected} {...props} />
)

const BHEdit = () => {
  return (
    <svg width="16px" height="16px" viewBox="0 0 10 10">
      <defs>
        <filter colorInterpolationFilters="auto" id="filter-1">
          <feColorMatrix
            in="SourceGraphic"
            type="matrix"
            values="0 0 0 0 1.000000 0 0 0 0 1.000000 0 0 0 0 1.000000 0 0 0 1.000000 0"
          ></feColorMatrix>
        </filter>
      </defs>
      <g
        id="页面-1"
        stroke="none"
        strokeWidth="1"
        fill="none"
        fillRule="evenodd"
      >
        <g id="团队页" transform="translate(-103.000000, -15.000000)">
          <g id="编组-3" transform="translate(103.000000, 15.000000)">
            <g filter="url(#filter-1)" id="编辑-(1)">
              <g>
                <rect
                  id="矩形"
                  fill="currentColor"
                  fillRule="nonzero"
                  opacity="0"
                  x="0"
                  y="0"
                  width="10"
                  height="10"
                ></rect>
                <path
                  d="M8.45732282,1.09636779 C8.53657097,1.01431612 8.65392554,0.981409161 8.7642812,1.01029473 C8.87463686,1.03918031 8.96081969,1.12536314 8.98970527,1.2357188 C9.01859084,1.34607446 8.98568388,1.46342903 8.90363221,1.54267718 L4.44022265,6.00619195 C4.31694866,6.12946593 4.11708204,6.12946593 3.99380805,6.00619195 C3.87053407,5.88291796 3.87053407,5.68305134 3.99380805,5.55977735 L8.45732282,1.096473 L8.45732282,1.09636779 Z M8.36484145,4.37067109 C8.36484145,4.1963501 8.50615653,4.05503503 8.68047751,4.05503503 C8.8547985,4.05503503 8.99611358,4.1963501 8.99611358,4.37067109 L8.99611358,7.84266777 C8.99611358,8.4818308 8.47794438,9 7.83878135,9 L2.15733223,9 C1.5181692,9 1,8.4818308 1,7.84266777 L1,2.16121865 C1,1.52205562 1.5181692,1.00388642 2.15733223,1.00388642 L5.52411689,1.00388642 C5.69843787,1.00388642 5.83975295,1.1452015 5.83975295,1.31952249 C5.83975295,1.49384347 5.69843787,1.63515855 5.52411689,1.63515855 L2.15733223,1.63515855 C1.86679725,1.63515855 1.63127212,1.87068368 1.63127212,2.16121865 L1.63127212,7.84266777 C1.63127212,8.13320275 1.86679725,8.36872788 2.15733223,8.36872788 L7.83878135,8.36872788 C8.12931632,8.36872788 8.36484145,8.13320275 8.36484145,7.84266777 L8.36484145,4.37067109 Z"
                  id="形状"
                  fill="currentColor"
                  fillRule="nonzero"
                ></path>
              </g>
            </g>
            <rect
              id="矩形"
              fillOpacity="0"
              fill="currentColor"
              x="0"
              y="0"
              width="10"
              height="10"
            ></rect>
          </g>
        </g>
      </g>
    </svg>
  )
}
const FileTreeShow = () => {
  return (
    <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" >
      <defs>
          <filter colorInterpolationFilters="auto" id="filter-1">
              <feColorMatrix in="SourceGraphic" type="matrix" values="0 0 0 0 0.552941 0 0 0 0 0.600000 0 0 0 0 0.643137 0 0 0 1.000000 0"></feColorMatrix>
          </filter>
      </defs>
      <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
          <g id="1月14.界面" transform="translate(-339.000000, -50.000000)">
              <g id="编组-29" transform="translate(339.000000, 50.000000)">
                  <rect id="矩形" fillOpacity="0" fill="#FFFFFF" x="0" y="3.97903932e-13" width="16" height="16"></rect>
                  <g filter="url(#filter-1)" id="收起">
                      <g transform="translate(8.000000, 8.000000) rotate(-90.000000) translate(-8.000000, -8.000000) translate(0.000000, 0.000000)">
                          <path d="M16,8 C16,3.58167598 12.418324,0 8,0 C3.58167598,0 0,3.58167598 0,8 C0,12.418324 3.58167598,16 8,16 C12.418324,16 16,12.418324 16,8 Z M11.7055642,10.2491173 C11.4041564,10.4786592 10.9738547,10.4205587 10.7446704,10.1189721 L8,6.51405587 L5.25532961,10.1189721 C5.02614525,10.4205587 4.59548603,10.4786592 4.29443575,10.2491173 C3.99302793,10.0197542 3.9347486,9.58945251 4.1642905,9.28822346 L7.45439106,4.9667933 C7.45832402,4.96178771 7.46332961,4.95821229 7.46744134,4.95338547 C7.48907263,4.92639106 7.51410056,4.90189944 7.54020112,4.87812291 C7.55253631,4.86686034 7.56379888,4.8541676 7.57684916,4.84397765 C7.5797095,4.8418324 7.58167598,4.83897207 7.58453631,4.83664804 C7.61903911,4.81036872 7.65550838,4.78873743 7.69305028,4.76996648 C7.70252514,4.76531844 7.71217877,4.76192179 7.7218324,4.75781006 C7.75651397,4.74243575 7.79173184,4.72992179 7.8276648,4.7206257 C7.83928492,4.71758659 7.85072626,4.71454749 7.86270391,4.71204469 C7.90811173,4.7027486 7.95369832,4.69649162 7.99964246,4.69649162 L8.00035754,4.69649162 C8.04630168,4.69649162 8.09188827,4.7027486 8.13729609,4.71204469 C8.14909497,4.71454749 8.16053631,4.71758659 8.1723352,4.7206257 C8.20826816,4.72992179 8.2436648,4.74243575 8.2781676,4.75781006 C8.28782123,4.76210056 8.29747486,4.76531844 8.30694972,4.76996648 C8.34431285,4.78873743 8.38078212,4.81036872 8.41546369,4.83664804 C8.41832402,4.8387933 8.4202905,4.84165363 8.42315084,4.84379888 C8.43620112,4.8541676 8.44746369,4.86668156 8.45979888,4.87812291 C8.48589944,4.90189944 8.5107486,4.92639106 8.53255866,4.95338547 C8.53649162,4.95839106 8.54167598,4.96178771 8.54560894,4.9667933 L11.8357095,9.28822346 C12.0650726,9.58945251 12.0067933,10.0197542 11.7055642,10.2491173 L11.7055642,10.2491173 Z" id="形状" fill="currentColor" fillRule="nonzero"></path>
                      </g>
                  </g>
              </g>
          </g>
      </g>
    </svg>
  )
}
const BHEditIcon = (props) => {
  return <Icon component={BHEdit} {...props} />
}


const toBreakOf = () => (
  <svg width="12px" height="12px" viewBox="0 0 10 10" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
        <g id="数据源（已数据源）备份" transform="translate(-137.000000, -350.000000)">
            <g id="编组-22备份-2" transform="translate(137.000000, 349.000000)">
                <rect
                  id="矩形"
                  x="2"
                  y="2"
                  width="10"
                  height="10"
                  ></rect>
                <g id="编组-21" transform="translate(0.000000, 1.000000)" stroke="currentColor" strokeLinecap="round">
                    <path d="M5,2.39366311 L5.43438948,1.95927363 L6.09049467,1.30316844 C6.81021473,0.583448385 7.9771115,0.583448385 8.69683156,1.30316844 C9.41655161,2.0228885 9.41655161,3.18978527 8.69683156,3.90950533 L8.04072637,4.56561052 L8.04072637,4.56561052 L7.60633689,5" id="矩形"></path>
                    <path d="M5,7.60633689 L4.56561052,8.04072637 L3.90950533,8.69683156 C3.18978527,9.41655161 2.0228885,9.41655161 1.30316844,8.69683156 C0.583448385,7.9771115 0.583448385,6.81021473 1.30316844,6.09049467 L1.95927363,5.43438948 L1.95927363,5.43438948 L2.39366311,5" id="矩形备份-2"></path>
                    <line x1="3" y1="3" x2="6.95602903" y2="7.10678663" id="路径-34"></line>
                </g>
            </g>
        </g>
    </g>
  </svg>
)
const BHToBreakOf = (props) => <Icon component={toBreakOf} {...props} />

const toLink = () => (
  <svg width="12px" height="12px" viewBox="0 0 12 12" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
        <g id="数据源（已数据源）备份" transform="translate(-474.000000, -349.000000)">
            <g id="编组-22" transform="translate(474.000000, 349.000000)">
                <rect id="矩形" fill="#FFFFFF" opacity="0" x="0" y="0" width="12" height="12"></rect>
                <g id="编组-21" stroke="currentColor" strokeLinecap="round">
                    <path d="M4.92045075,3.38921118 L5.5355071,2.77415483 L6.4644929,1.84516903 C7.48355162,0.826110315 9.13577225,0.826110315 10.154831,1.84516903 C11.1738897,2.86422775 11.1738897,4.51644838 10.154831,5.5355071 L9.22584517,6.4644929 L9.22584517,6.4644929 L8.61078882,7.07954925" id="矩形"></path>
                    <path d="M7.07954925,8.61078882 L6.4644929,9.22584517 L5.5355071,10.154831 C4.51644838,11.1738897 2.86422775,11.1738897 1.84516903,10.154831 C0.826110315,9.13577225 0.826110315,7.48355162 1.84516903,6.4644929 L2.77415483,5.5355071 L2.77415483,5.5355071 L3.38921118,4.92045075" id="矩形备份-2"></path>
                    <line x1="4.4240855" y1="7.5759145" x2="7.81819805" y2="4.18180195" id="路径-34"></line>
                </g>
            </g>
        </g>
    </g>
  </svg>
)
const BHToLink = (props) => <Icon component={toLink} {...props} />

const openNewPage = () => (
  <svg width="14px" height="14px" viewBox="0 0 12 12" version="1.1">
      <defs>
          <filter colorInterpolationFilters="auto" id="filter-1">
              <feColorMatrix in="SourceGraphic" type="matrix" values="0 0 0 0 1.000000 0 0 0 0 1.000000 0 0 0 0 1.000000 0 0 0 1.000000 0"></feColorMatrix>
          </filter>
      </defs>
      <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
          <g id="个人账户" transform="translate(-1418.000000, -45.000000)">
              <g id="编组-8" transform="translate(1418.000000, 45.000000)">
                  <rect id="矩形" fill="#FFFFFF" opacity="0" x="0" y="0" width="12" height="12"></rect>
                  <g filter="url(#filter-1)" id="编组-7">
                      <g transform="translate(1.500000, 1.200000)">
                          <path stroke="currentColor" d="M0,4.00819724 L0,1.35152913 C-6.76353751e-17,0.799244385 0.44771525,0.351529135 1,0.351529135 L8,0.351529135 C8.55228475,0.351529135 9,0.799244385 9,1.35152913 L9,8.3 C9,8.85228475 8.55228475,9.3 8,9.3 L5.42256872,9.3 L5.42256872,9.3" id="矩形备份-11" transform="translate(4.500000, 4.825765) rotate(-180.000000) translate(-4.500000, -4.825765) "></path>
                          <polyline id="矩形备份-12" stroke="currentColor" transform="translate(7.000000, 2.340078) rotate(-90.000000) translate(-7.000000, -2.340078) " points="8.98854908 0.340078216 8.98854908 4.34007822 5.01145092 4.34007822"></polyline>
                          <line x1="9.96485418" y1="2.82225263" x2="2.98489329" y2="2.80221353" id="矩形备份-13" stroke="currentColor" transform="translate(6.474874, 2.812233) rotate(-225.000000) translate(-6.474874, -2.812233) "></line>
                      </g>
                  </g>
              </g>
          </g>
      </g>
  </svg>
)

const BTOpenNewPage = (props) => <Icon component={openNewPage} {...props} />

const BHStorageService = () => (
<svg width="14px" height="11px" viewBox="0 0 14 11" version="1.1">
    <g id="全局搜索" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
        <g id="历史版本icon" transform="translate(-144.000000, -444.000000)" stroke="#8D99A4">
            <g id="编组-8" transform="translate(144.500000, 445.418955)">
                <path d="M0,8.41913857 C3.82249061,8.41913857 6.17692738,8.41913857 7.06331031,8.41913857 C8.39288471,8.41913857 9.47071772,7.34130556 9.47071772,6.01173116 C9.47071772,4.68215676 8.39288471,3.60432375 7.06331031,3.60432375 C5.73373591,3.60432375 6.72611386,3.60432375 6.10034735,3.60432375" id="椭圆形" strokeLinecap="round" transform="translate(4.735359, 6.011731) scale(-1, 1) translate(-4.735359, -6.011731) "></path>
                <path d="M10.1111111,3.60432375 C10.1111111,1.47700472 8.6021449,-1.86517468e-14 6.74074074,-1.86517468e-14 C4.87933658,-1.86517468e-14 3.37037037,1.47700472 3.37037037,3.60432375" id="椭圆形"></path>
                <path d="M12.2023245,8.41913857 C12.696559,7.90787069 13,7.21560878 13,6.45344991 C13,4.87992098 11.7066004,3.60432375 10.1111111,3.60432375" id="椭圆形备份-4" strokeLinecap="round"></path>
            </g>
        </g>
    </g>
</svg>
)
const BHStorageServiceIcon = (props) => <Icon component={BHStorageService} {...props} />

const disconnectDatabase = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title></title>
    <defs>
        <filter colorInterpolationFilters="auto" id="filter-1">
            <feColorMatrix in="SourceGraphic" type="matrix" values="0 0 0 0 0.921569 0 0 0 0 0.294118 0 0 0 0 0.376471 0 0 0 1.000000 0"></feColorMatrix>
        </filter>
    </defs>
    <g id="全局搜索" stroke="none" strokeWidth="1" fill="red" fillRule="evenodd">
        <g id="单元格工具栏" transform="translate(-361.000000, -112.000000)">
            <g id="编组-13" transform="translate(361.000000, 112.000000)">
                <g filter="url(#filter-1)" id="断开链接备份">
                    <g>
                        <rect id="矩形" fill="red" fillRule="nonzero" opacity="0" x="0" y="0" width="14" height="14"></rect>
                        <path d="M9.65035451,4.99130258 C9.44483208,4.99120915 9.25939738,4.86786634 9.1798361,4.67832226 C9.10027482,4.48877817 9.1421021,4.26999553 9.28596783,4.12318744 L11.7249975,1.63696754 C11.8527864,1.50675303 12.0403199,1.45462104 12.216956,1.50020917 C12.3935921,1.5457973 12.5324955,1.68217963 12.5813427,1.85798227 C12.6301899,2.03378491 12.5815597,2.22229924 12.4537708,2.35251375 L10.0172938,4.83937197 C9.92049145,4.93747053 9.78815292,4.9922605 9.65035451,4.99130258 Z M1.96696448,12.8239998 C1.76116541,12.8241715 1.57538611,12.7007288 1.49576749,12.5109086 C1.41614888,12.3210885 1.45826108,12.1020119 1.6025778,11.9552589 L5.14625428,8.34369733 C5.27404319,8.21348281 5.46157674,8.16135081 5.63821284,8.20693894 C5.81484894,8.25252707 5.95375236,8.38890941 6.00259953,8.56471205 C6.0514467,8.7405147 6.00281658,8.92902904 5.87502765,9.05924354 L2.33135117,12.6708051 C2.2353812,12.7687379 2.10406444,12.8239458 1.96696448,12.8239998 Z" id="形状" fill="red" fillRule="nonzero"></path>
                        <path d="M7.168,4.21703755 L7.78119097,3.60387204 C8.59723916,2.79337599 9.91536503,2.79337599 10.7314132,3.60387204 C11.5428623,4.41925492 11.5428623,5.73615463 10.7314132,6.55153751 L10.0862519,7.196" id="路径" fill="red" fillRule="nonzero"></path>
                        <path d="M10.1011991,7.728 C9.89377637,7.72815787 9.70670527,7.60331648 9.62730592,7.41174332 C9.54790657,7.22017016 9.59183512,6.9996401 9.73858684,6.853091 L10.3850105,6.20940271 C10.7909072,5.81238862 10.9521212,5.22795682 10.8071491,4.6790686 C10.662177,4.13018038 10.2333799,3.70149838 9.68434438,3.5565652 C9.13530883,3.41163201 8.55072017,3.57280269 8.15359952,3.97859046 L7.53920873,4.59281639 C7.40977428,4.72233053 7.2210584,4.77297259 7.04414853,4.72566621 C6.86723867,4.67835984 6.72901165,4.54029198 6.68153624,4.36347147 C6.63406083,4.18665095 6.68454968,3.99794103 6.81398413,3.86842689 L7.42837493,3.25420095 C8.44491563,2.23793302 10.0930538,2.23793302 11.1095945,3.25420095 C12.1261352,4.27046889 12.1261352,5.91816476 11.1095945,6.9344327 L10.4638114,7.58004245 C10.367298,7.67540534 10.2368952,7.72861199 10.1011991,7.728 L10.1011991,7.728 Z" id="路径" fill="red" fillRule="nonzero"></path>
                        <path d="M7.476,9.81364224 L6.56344054,10.7300467 C5.74534076,11.5433178 4.42405304,11.5433178 3.60595326,10.7300467 C2.79268225,9.91194696 2.79268225,8.59065925 3.60595326,7.77255947 L4.51915355,6.86" id="路径" fill="red" fillRule="nonzero"></path>
                        <path d="M5.08737643,11.844 C4.03830271,11.8459864 3.09160709,11.2144589 2.68970518,10.2445368 C2.28780327,9.27461462 2.51005929,8.15782996 3.25260719,7.41608544 L4.16329092,6.50584828 C4.36260559,6.30617536 4.68590137,6.30603238 4.88539227,6.50552891 C5.08488316,6.70502544 5.08502602,7.02861622 4.88571136,7.22828914 L3.97630399,8.13852631 C3.37389365,8.75481126 3.37928073,9.7415878 3.98838384,10.3512467 C4.59748695,10.9609056 5.58336392,10.9662976 6.19908705,10.3633375 L7.1084944,9.45054533 C7.23754238,9.3214937 7.4255731,9.27115417 7.6017572,9.31848916 C7.7779413,9.36582415 7.91551222,9.50364235 7.96264834,9.68002897 C8.00978445,9.8564156 7.95932466,10.0445733 7.83027667,10.173625 L6.9208693,11.0838621 C6.43567607,11.57219 5.77544497,11.845908 5.08737643,11.844 L5.08737643,11.844 Z" id="路径" fill="red" fillRule="nonzero"></path>
                        <path d="M4.536,6.87942075 L7.48633371,9.828 M7.18899839,4.228 L10.108,7.20853178" id="形状" fill="red" fillRule="nonzero"></path>
                        <path d="M5.46579958,8.24994586 C5.25938737,8.24933551 5.0735873,8.1247229 4.99476364,7.93403093 C4.91593998,7.74333897 4.95955484,7.52397392 5.105335,7.37790186 L6.3132748,6.16917862 C6.5137658,5.96965174 6.83811057,5.97036681 7.03771913,6.17077577 C7.23732768,6.37118473 7.23661232,6.69539679 7.03612132,6.89492366 L5.82690328,8.10045261 C5.73109093,8.19611929 5.6012234,8.24988312 5.46579958,8.24994586 L5.46579958,8.24994586 Z M6.73573417,9.51999984 C6.52880875,9.52016245 6.34218622,9.39563796 6.26297726,9.20455108 C6.18376829,9.0134642 6.22759151,8.79349393 6.37399135,8.64731683 L7.58320939,7.43859358 C7.71244765,7.30952232 7.90075562,7.25917514 8.07719949,7.30651733 C8.25364337,7.35385952 8.39141714,7.49169868 8.43862275,7.66811213 C8.48582837,7.84452558 8.43529417,8.03271192 8.30605591,8.16178318 L7.09747698,9.36986756 C7.00161269,9.46592502 6.87147124,9.51993691 6.73573417,9.51999984 L6.73573417,9.51999984 Z" id="形状" fill="red" fillRule="nonzero"></path>
                    </g>
                </g>
                <rect id="矩形" fillOpacity="0" fill="red" x="0" y="0" width="14" height="14"></rect>
            </g>
        </g>
    </g>
  </svg>
)
const BHDisconnectDatabase = (props) => <Icon component={disconnectDatabase} {...props} />

// 运行上方所有cell
const BHRunPreCells = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g id="全局搜索" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="单元格工具栏" transform="translate(-855.000000, -210.000000)">
        <g id="编组-15" transform="translate(855.000000, 210.000000)">
          <rect id="矩形" fillOpacity="0" fill="#FFFFFF" x="0" y="0" width="16" height="16"></rect>
          <g id="编组-10备份" fill="currentColor" fillRule="nonzero">
            <g id="编组-15备份-3">
              <path d="M1.85281552,0.196941272 C1.49786129,-0.0309205264 1.02814292,-0.0636341704 0.637127963,0.112274433 C0.246113007,0.288183036 0,0.642960145 0,1.0304802 L0,11.9705498 C0.000377515361,12.3578845 0.246719957,12.7123074 0.637647517,12.8879593 C1.02857508,13.0636112 1.49803108,13.0308115 1.85281552,12.8030584 L9.51907145,7.33353893 C9.82121188,7.13967725 10,6.82980407 10,6.5 C10,6.17019593 9.82121188,5.86032275 9.51907145,5.66646107 L1.85281552,0.196941272 Z" id="路径备份-10"></path>
            </g>
            <g id="向上箭头-1" transform="translate(12.000000, 9.571429) scale(1, -1) rotate(-180.000000) translate(-12.000000, -9.571429) translate(8.000000, 5.000000)">
              <rect id="矩形" opacity="0" x="0" y="0" width="8" height="9.14285714"></rect>
              <path d="M4.28813749,1.65014927 C4.13171102,1.48169628 3.87814371,1.48169628 3.72171725,1.65014927 L1.45563568,4.09119098 C1.35150222,4.19953167 1.30973932,4.35996789 1.34639858,4.51083587 C1.38305784,4.66170384 1.49243423,4.77952498 1.63248883,4.81901464 C1.77254344,4.8585043 1.9214805,4.81351696 2.02205593,4.70134354 L3.60434728,2.99688627 L3.60434728,8.71134897 C3.60434728,8.94966435 3.78369309,9.14285714 4.00492737,9.14285714 C4.22616164,9.14285714 4.40550746,8.94966435 4.40550746,8.71134897 L4.40550746,2.99688627 L5.98779881,4.70134354 C6.14497798,4.86487311 6.39482212,4.86253439 6.54933938,4.69608712 C6.70385665,4.52963985 6.70602774,4.26050569 6.55421906,4.09119098 L4.28813749,1.65014927 L4.28813749,1.65014927 Z" id="路径"></path>
            </g>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const BHRunPreCellsIcon = (props) => <Icon component={BHRunPreCells} {...props} />

// 运行下方所有cell
const BHRunNextCells = () => (
  <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
    <g id="全局搜索" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="单元格工具栏" transform="translate(-878.000000, -210.000000)">
        <g id="编组-13" transform="translate(878.000000, 210.000000)">
          <rect id="矩形" fillOpacity="0" fill="#FFFFFF" x="0" y="0" width="16" height="16"></rect>
          <g id="编组-10" fill="currentColor" fillRule="nonzero">
            <g id="编组-15备份-3">
              <path d="M1.85281552,0.196941272 C1.49786129,-0.0309205264 1.02814292,-0.0636341704 0.637127963,0.112274433 C0.246113007,0.288183036 0,0.642960145 0,1.0304802 L0,11.9705498 C0.000377515361,12.3578845 0.246719957,12.7123074 0.637647517,12.8879593 C1.02857508,13.0636112 1.49803108,13.0308115 1.85281552,12.8030584 L9.51907145,7.33353893 C9.82121188,7.13967725 10,6.82980407 10,6.5 C10,6.17019593 9.82121188,5.86032275 9.51907145,5.66646107 L1.85281552,0.196941272 Z" id="路径备份-10"></path>
            </g>
            <g id="向上箭头-1" transform="translate(12.000000, 11.071429) rotate(-180.000000) translate(-12.000000, -11.071429) translate(8.000000, 6.500000)">
              <rect id="矩形" opacity="0" x="0" y="0" width="8" height="9.14285714"></rect>
              <path d="M4.28813749,1.65014927 C4.13171102,1.48169628 3.87814371,1.48169628 3.72171725,1.65014927 L1.45563568,4.09119098 C1.35150222,4.19953167 1.30973932,4.35996789 1.34639858,4.51083587 C1.38305784,4.66170384 1.49243423,4.77952498 1.63248883,4.81901464 C1.77254344,4.8585043 1.9214805,4.81351696 2.02205593,4.70134354 L3.60434728,2.99688627 L3.60434728,8.71134897 C3.60434728,8.94966435 3.78369309,9.14285714 4.00492737,9.14285714 C4.22616164,9.14285714 4.40550746,8.94966435 4.40550746,8.71134897 L4.40550746,2.99688627 L5.98779881,4.70134354 C6.14497798,4.86487311 6.39482212,4.86253439 6.54933938,4.69608712 C6.70385665,4.52963985 6.70602774,4.26050569 6.55421906,4.09119098 L4.28813749,1.65014927 L4.28813749,1.65014927 Z" id="路径"></path>
            </g>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const BHRunNextCellsIcon = (props) => <Icon component={BHRunNextCells} {...props} />

// cell恢复运行
const BHCellResume = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <defs>
      <circle id="path-4" cx="12" cy="12" r="11.5">
        <animateTransform
          attributeName="transform"
          attributeType="XML"
          type="rotate"
          from="0 12 12"
          to="360 12 12"
          dur="1.5s"
          repeatCount="indefinite"
        />
      </circle>
      <mask
        id="mask-4"
        maskContentUnits="userSpaceOnUse"
        maskUnits="objectBoundingBox"
        x="0"
        y="0"
        width="23"
        height="23"
        fill="white"
      >
        <use xlinkHref="#path-4"></use>
      </mask>
    </defs>
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="文件" transform="translate(-384.000000, -215.000000)" fill="#8D99A4" fillRule="nonzero">
        <g id="编组-19" transform="translate(384.000000, 215.000000)">
          <path d="M17.1959791,9.01235514 C16.9716557,8.91595173 16.7121081,9.02162469 16.6157047,9.24594801 L16.3524492,9.86700845 C15.5348741,8.1706792 13.7959049,7.02681564 11.8474437,7.02681564 C9.28162979,7.02681564 7.31092159,8.77320051 6.94755489,11.3723848 C6.91418448,11.6152472 7.08289045,11.8377166 7.32389897,11.8710871 C7.344292,11.8747949 7.36468503,11.8747949 7.38507805,11.8747949 C7.60198573,11.8747949 7.79108472,11.7153585 7.82260124,11.4928891 C8.12664277,9.31639667 9.70617558,7.91113156 11.8455897,7.91113156 C13.4733243,7.91113156 14.9230833,8.8807274 15.5812219,10.3119473 L14.7951633,9.97824315 C14.57084,9.88183974 14.3112924,9.98751271 14.214889,10.211836 C14.1184856,10.4361594 14.2241585,10.695707 14.4484818,10.7921104 L16.1318337,11.5058664 C16.1336876,11.5058664 16.1336876,11.5058664 16.1355415,11.5077204 C16.146665,11.5132821 16.1596424,11.515136 16.1707659,11.5188438 C16.2152597,11.5336751 16.2597536,11.5429447 16.3042475,11.5429447 C16.3339101,11.5429447 16.3635727,11.5392369 16.3932353,11.5336751 C16.4006509,11.5318212 16.4080666,11.5281134 16.4173361,11.5262595 C16.4210439,11.5244056 16.4266057,11.5262595 16.4303135,11.5244056 C16.4377292,11.5225516 16.4432909,11.5188438 16.4507065,11.515136 C16.4692457,11.5095743 16.4859309,11.5021586 16.5026161,11.4928891 C16.5118856,11.4873273 16.5230091,11.4817656 16.5322787,11.4762039 C16.5489639,11.4650804 16.5656491,11.4539569 16.5804804,11.4428334 C16.5897499,11.4354178 16.5971656,11.4298561 16.6045812,11.4224404 C16.6194125,11.4076091 16.6342438,11.3927778 16.6472212,11.3760926 C16.6527829,11.368677 16.6601986,11.3612613 16.6639064,11.3538457 C16.6805916,11.3297448 16.6972768,11.3019362 16.7084003,11.2741275 L16.7084003,11.2722736 C16.7102542,11.2704197 16.7102542,11.2667118 16.7121081,11.2648579 L17.4240102,9.58892169 C17.5259753,9.36830619 17.4203024,9.10690464 17.1959791,9.01235514 L17.1959791,9.01235514 Z" id="路径"></path>
          <path d="M16.3709883,12.1806903 C16.1299798,12.145466 15.9056565,12.3160259 15.8722861,12.5570344 C15.5663906,14.7335268 13.9887117,16.1387919 11.8492976,16.1387919 C10.221563,16.1387919 8.77180405,15.1691961 8.11366539,13.7379762 L8.89972397,14.0716803 C8.95534132,14.0957812 9.01466649,14.1069046 9.07213775,14.1069046 C9.24455154,14.1069046 9.40769578,14.0049395 9.47999834,13.8380874 C9.57640175,13.6137641 9.47072878,13.3542165 9.24640546,13.2578131 L7.5630536,12.544057 C7.56119969,12.544057 7.55934577,12.544057 7.55934577,12.5422031 C7.55193012,12.5384953 7.54266056,12.5384953 7.53524491,12.5347875 C7.52597535,12.5329336 7.51670579,12.5292257 7.50743623,12.5255179 L7.50558233,12.5255179 C7.48148148,12.5181023 7.45738061,12.5143944 7.43142586,12.5125405 L7.42401021,12.5125405 C7.40176327,12.5106866 7.37766242,12.5106866 7.35356155,12.5125405 C7.34985373,12.5125405 7.344292,12.5125405 7.34058419,12.5143944 C7.3276068,12.5162483 7.31462942,12.5162483 7.29979813,12.5181023 C7.29052857,12.5199562 7.28311292,12.523664 7.27569726,12.5255179 C7.27013553,12.5273718 7.2645738,12.5292257 7.26086597,12.5310796 C7.24047295,12.5366414 7.22007991,12.544057 7.2015408,12.5533266 L7.19041733,12.5588883 C7.17002431,12.5681579 7.15148518,12.5792813 7.13294606,12.5922587 C7.12923824,12.5941126 7.12738433,12.5978205 7.12553041,12.5996744 C7.08845218,12.6274831 7.05508177,12.6608535 7.029127,12.6979317 C7.0272731,12.6997856 7.02541918,12.7034934 7.02356527,12.7053473 C7.01244181,12.7220326 7.00317225,12.7405717 6.9957566,12.7591108 C6.99204877,12.7665264 6.98834094,12.7720882 6.98463312,12.7776499 L6.98463312,12.7795038 C6.98463312,12.7813577 6.98277921,12.7832116 6.98277921,12.7850656 L6.27087709,14.4610018 C6.17447368,14.6853251 6.28014665,14.9448727 6.50446998,15.0412762 C6.56008733,15.065377 6.61941251,15.0765005 6.67688378,15.0765005 C6.84929757,15.0765005 7.01244181,14.9745353 7.08474435,14.8076833 L7.34799982,14.1866228 C8.16557491,15.8829521 9.90454412,17.0268156 11.8530054,17.0268156 C14.4188192,17.0268156 16.3895274,15.2804308 16.7528941,12.6812465 C16.7807028,12.4383841 16.6119969,12.2140607 16.3709883,12.1806903 L16.3709883,12.1806903 Z" id="路径"></path>
        </g>
      </g>
      <use
        stroke="#979797"
        mask="url(#mask-4)"
        strokeWidth="4"
        strokeDasharray="4,2"
        xlinkHref="#path-4"
      ></use>
    </g>
  </svg>
)
const BHCellResumeIcon = (props) => <Icon component={BHCellResume} {...props} />

// 恢复运行
const BHResumeRun = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="文件" transform="translate(-384.000000, -215.000000)" fill="#8D99A4" fillRule="nonzero">
        <g id="编组-19" transform="translate(384.000000, 215.000000)">
          <g id="运行备份">
            <rect id="矩形" opacity="0" x="0" y="0" width="24" height="24"></rect>
            <path d="M12,2 C6.48615385,2 2,6.48615385 2,12 C2,17.5138462 6.48615385,22 12,22 C17.5138462,22 22,17.5138462 22,12 C22,6.48615385 17.5138462,2 12,2 Z M12,20.4615385 C7.33461538,20.4615385 3.53846154,16.6653846 3.53846154,12 C3.53846154,7.33461538 7.33461538,3.53846154 12,3.53846154 C16.6653846,3.53846154 20.4615385,7.33461538 20.4615385,12 C20.4615385,16.6653846 16.6653846,20.4615385 12,20.4615385 Z" id="形状"></path>
          </g>
          <path d="M17.1959791,9.01235514 C16.9716557,8.91595173 16.7121081,9.02162469 16.6157047,9.24594801 L16.3524492,9.86700845 C15.5348741,8.1706792 13.7959049,7.02681564 11.8474437,7.02681564 C9.28162979,7.02681564 7.31092159,8.77320051 6.94755489,11.3723848 C6.91418448,11.6152472 7.08289045,11.8377166 7.32389897,11.8710871 C7.344292,11.8747949 7.36468503,11.8747949 7.38507805,11.8747949 C7.60198573,11.8747949 7.79108472,11.7153585 7.82260124,11.4928891 C8.12664277,9.31639667 9.70617558,7.91113156 11.8455897,7.91113156 C13.4733243,7.91113156 14.9230833,8.8807274 15.5812219,10.3119473 L14.7951633,9.97824315 C14.57084,9.88183974 14.3112924,9.98751271 14.214889,10.211836 C14.1184856,10.4361594 14.2241585,10.695707 14.4484818,10.7921104 L16.1318337,11.5058664 C16.1336876,11.5058664 16.1336876,11.5058664 16.1355415,11.5077204 C16.146665,11.5132821 16.1596424,11.515136 16.1707659,11.5188438 C16.2152597,11.5336751 16.2597536,11.5429447 16.3042475,11.5429447 C16.3339101,11.5429447 16.3635727,11.5392369 16.3932353,11.5336751 C16.4006509,11.5318212 16.4080666,11.5281134 16.4173361,11.5262595 C16.4210439,11.5244056 16.4266057,11.5262595 16.4303135,11.5244056 C16.4377292,11.5225516 16.4432909,11.5188438 16.4507065,11.515136 C16.4692457,11.5095743 16.4859309,11.5021586 16.5026161,11.4928891 C16.5118856,11.4873273 16.5230091,11.4817656 16.5322787,11.4762039 C16.5489639,11.4650804 16.5656491,11.4539569 16.5804804,11.4428334 C16.5897499,11.4354178 16.5971656,11.4298561 16.6045812,11.4224404 C16.6194125,11.4076091 16.6342438,11.3927778 16.6472212,11.3760926 C16.6527829,11.368677 16.6601986,11.3612613 16.6639064,11.3538457 C16.6805916,11.3297448 16.6972768,11.3019362 16.7084003,11.2741275 L16.7084003,11.2722736 C16.7102542,11.2704197 16.7102542,11.2667118 16.7121081,11.2648579 L17.4240102,9.58892169 C17.5259753,9.36830619 17.4203024,9.10690464 17.1959791,9.01235514 L17.1959791,9.01235514 Z" id="路径"></path>
          <path d="M16.3709883,12.1806903 C16.1299798,12.145466 15.9056565,12.3160259 15.8722861,12.5570344 C15.5663906,14.7335268 13.9887117,16.1387919 11.8492976,16.1387919 C10.221563,16.1387919 8.77180405,15.1691961 8.11366539,13.7379762 L8.89972397,14.0716803 C8.95534132,14.0957812 9.01466649,14.1069046 9.07213775,14.1069046 C9.24455154,14.1069046 9.40769578,14.0049395 9.47999834,13.8380874 C9.57640175,13.6137641 9.47072878,13.3542165 9.24640546,13.2578131 L7.5630536,12.544057 C7.56119969,12.544057 7.55934577,12.544057 7.55934577,12.5422031 C7.55193012,12.5384953 7.54266056,12.5384953 7.53524491,12.5347875 C7.52597535,12.5329336 7.51670579,12.5292257 7.50743623,12.5255179 L7.50558233,12.5255179 C7.48148148,12.5181023 7.45738061,12.5143944 7.43142586,12.5125405 L7.42401021,12.5125405 C7.40176327,12.5106866 7.37766242,12.5106866 7.35356155,12.5125405 C7.34985373,12.5125405 7.344292,12.5125405 7.34058419,12.5143944 C7.3276068,12.5162483 7.31462942,12.5162483 7.29979813,12.5181023 C7.29052857,12.5199562 7.28311292,12.523664 7.27569726,12.5255179 C7.27013553,12.5273718 7.2645738,12.5292257 7.26086597,12.5310796 C7.24047295,12.5366414 7.22007991,12.544057 7.2015408,12.5533266 L7.19041733,12.5588883 C7.17002431,12.5681579 7.15148518,12.5792813 7.13294606,12.5922587 C7.12923824,12.5941126 7.12738433,12.5978205 7.12553041,12.5996744 C7.08845218,12.6274831 7.05508177,12.6608535 7.029127,12.6979317 C7.0272731,12.6997856 7.02541918,12.7034934 7.02356527,12.7053473 C7.01244181,12.7220326 7.00317225,12.7405717 6.9957566,12.7591108 C6.99204877,12.7665264 6.98834094,12.7720882 6.98463312,12.7776499 L6.98463312,12.7795038 C6.98463312,12.7813577 6.98277921,12.7832116 6.98277921,12.7850656 L6.27087709,14.4610018 C6.17447368,14.6853251 6.28014665,14.9448727 6.50446998,15.0412762 C6.56008733,15.065377 6.61941251,15.0765005 6.67688378,15.0765005 C6.84929757,15.0765005 7.01244181,14.9745353 7.08474435,14.8076833 L7.34799982,14.1866228 C8.16557491,15.8829521 9.90454412,17.0268156 11.8530054,17.0268156 C14.4188192,17.0268156 16.3895274,15.2804308 16.7528941,12.6812465 C16.7807028,12.4383841 16.6119969,12.2140607 16.3709883,12.1806903 L16.3709883,12.1806903 Z" id="路径"></path>
        </g>
      </g>
    </g>
  </svg>
)
const BHResumeRunIcon = (props) => <Icon component={BHResumeRun} {...props} />

const BHTFBoard = () => (
  <svg width="22px" height="24px" viewBox="0 0 22 24" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组-8" transform="translate(0.792822, 0.440909)">
        <path d="M9.06932513,0.345820457 L9.06932513,20.6584309 C9.06932513,20.7688879 8.97978208,20.8584309 8.86932513,20.8584309 C8.83504698,20.8584309 8.80134412,20.8496208 8.77145098,20.8328461 L5.74081931,19.1321884 C5.67774401,19.0967934 5.63869345,19.030101 5.63869345,18.9577733 L5.63869345,6.92965951 C5.63869345,6.81920256 5.5491504,6.72965951 5.43869345,6.72965951 C5.40325732,6.72965951 5.36845742,6.73907457 5.33785508,6.75694118 L0.300838372,9.69770908 C0.205448667,9.75340057 0.0829731571,9.72121881 0.027281664,9.62582911 C0.00941505775,9.59522676 -6.89549716e-16,9.56042686 0,9.52499074 L0,5.33400911 C-8.76340173e-18,5.2624505 0.0382314698,5.19634438 0.100255788,5.16065659 L8.76958091,0.172467947 C8.86532086,0.117380738 8.98759043,0.150336295 9.04267764,0.246076243 C9.06013647,0.276419191 9.06932513,0.310813235 9.06932513,0.345820457 Z" id="路径-37" fill="currentColor"></path>
        <path d="M10.2988784,0.24984454 C10.3555125,0.15501142 10.4783009,0.12404497 10.573134,0.180679037 L10.573134,0.180679037 L19.0542467,5.24557517 C19.1146885,5.28167087 19.1517016,5.34688602 19.1517016,5.41728572 L19.1517016,5.41728572 L19.1517016,9.41792386 C19.1517016,9.4549254 19.1414369,9.4912016 19.1220494,9.52271724 C19.0641736,9.61679769 18.9409887,9.64614738 18.8469083,9.58827159 L18.8469083,9.58827159 L14.0889706,6.66131539 C14.057455,6.64192781 14.0211788,6.63166312 13.9841773,6.63166312 C13.8737203,6.63166312 13.7841773,6.72120617 13.7841773,6.83166312 L13.7841773,6.83166312 L13.7841773,8.90033666 C13.7841773,8.97234851 13.8228904,9.03880057 13.885534,9.074318 L13.885534,9.074318 L16.2987135,10.4425332 C16.3613571,10.4780506 16.4000702,10.5445026 16.4000702,10.6165145 L16.4000702,10.6165145 L16.4000702,14.4173048 C16.4000702,14.4511638 16.3914741,14.4844686 16.3750875,14.5140981 C16.32163,14.6107575 16.1999362,14.6457795 16.1032768,14.592322 L16.1032768,14.592322 L14.0809706,13.4738848 C14.0513411,13.4574982 14.0180363,13.4489021 13.9841773,13.4489021 C13.8737203,13.4489021 13.7841773,13.5384451 13.7841773,13.6489021 L13.7841773,13.6489021 L13.7833862,16.8369273 C13.170677,17.5323765 12.8053551,18.4726601 12.8053551,19.559091 C12.8053551,19.7258826 12.8139654,19.8892297 12.8307593,20.0487751 L10.5402389,20.8998236 C10.517951,20.9081037 10.4943652,20.9123432 10.470589,20.9123432 C10.360132,20.9123432 10.270589,20.8228002 10.270589,20.7123432 L10.270589,20.7123432 L10.270589,0.35238959 C10.270589,0.316280898 10.2803646,0.280845757 10.2988784,0.24984454 Z" id="形状结合" fill="currentColor"></path>
        <circle id="椭圆形备份-22" fill="currentColor" cx="17.2071785" cy="19.559091" r="4"></circle>
        <g id="编组-42" transform="translate(14.707178, 18.859091)" fill="#FFFFFF">
          <rect id="矩形" x="0" y="2" width="1" height="1.5" rx="0.200000003"></rect>
          <rect id="矩形备份-84" x="2" y="1" width="1" height="2.5" rx="0.200000003"></rect>
          <rect id="矩形备份-92" x="4" y="0" width="1" height="3.5" rx="0.200000003"></rect>
        </g>
        <g id="编组-43" transform="translate(17.116575, 18.299655) rotate(3.000000) translate(-17.116575, -18.299655) translate(15.012405, 16.686882)">
          <path d="M3.82737985,0.602866924 L3.61171792,0.380958561 L3.06546272,0.380958561 C2.99740409,0.38096898 2.93451085,0.344666114 2.90047853,0.28572734 C2.8664462,0.226788567 2.8664462,0.154169996 2.90047853,0.0952312229 C2.93451085,0.0362924496 2.99740409,2.27373675e-13 3.06546272,2.27373675e-13 L4.01785913,2.27373675e-13 C4.12305793,2.27373675e-13 4.20833841,0.0852804805 4.20833841,0.190479279 L4.20833841,1.14287568 C4.20833841,1.21093432 4.17204596,1.27382755 4.11310718,1.30785988 C4.05416841,1.34189221 3.98154984,1.34189221 3.92261107,1.30785988 C3.86367229,1.27382755 3.82736943,1.21093432 3.82737985,1.14287568 L3.82737985,0.602866924 Z" id="路径" fill="#FFFFFF" fillRule="nonzero"></path>
          <line x1="9.53017242e-15" y1="3.22554519" x2="3.83234343" y2="0.375711063" id="路径-39" stroke="#FFFFFF" strokeWidth="0.35" strokeLinecap="round"></line>
        </g>
      </g>
    </g>
  </svg>
)
const BHTFBoardIcon = (props) => <Icon component={BHTFBoard} {...props} />

const BHmonitor = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组-10">
        <rect id="矩形" fillOpacity="0" fill="#FFFFFF" x="0" y="0" width="24" height="24"></rect>
        <g id="编组-11" transform="translate(0.000000, 2.000000)">
          <rect id="矩形" fill="currentColor" x="0" y="0" width="24" height="16" rx="1"></rect>
          <rect id="矩形" fill="currentColor" x="7" y="19" width="10" height="1.5" rx="0.75"></rect>
          <rect id="矩形备份-3" fill="currentColor" transform="translate(12.000000, 17.250000) rotate(-270.000000) translate(-12.000000, -17.250000) " x="9.5" y="16.5" width="5" height="1.5" rx="0.75"></rect>
          <path d="M1.43134527,16.70957 L8.24931418,8.67212513 C8.28504064,8.63000857 8.34814487,8.62482836 8.39026143,8.66055482 C8.39053446,8.66078642 8.39080624,8.66101949 8.39107676,8.66125401 L11.6648392,11.49933 C11.7055479,11.534621 11.7668974,11.5313178 11.8035814,11.4918596 L16.4611872,6.48202773 L16.4611872,6.48202773" id="路径-2" stroke="#FFFFFF" strokeWidth="1.5"></path>
          <circle id="椭圆形" stroke="#FFFFFF" strokeWidth="1.5" cx="18" cy="5" r="2.25"></circle>
        </g>
      </g>
    </g>
  </svg>
)
const BHmonitorIcon = (props) => <Icon component={BHmonitor} {...props} />

const BHWandb = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1">
    <g id="页面-1备份" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="编组-13">
        <rect id="矩形" fillOpacity="0" fill="#FFFFFF" x="0" y="0" width="24" height="24"></rect>
        <g id="编组-12" transform="translate(0.500000, 0.500000)" fill="currentColor">
          <circle id="椭圆形" cx="2.5" cy="7.5" r="2.5"></circle>
          <circle id="椭圆形备份-21" cx="20.5" cy="7.5" r="2.5"></circle>
          <circle id="椭圆形备份-19" cx="2.5" cy="19.5" r="2.5"></circle>
          <circle id="椭圆形备份-28" cx="11.5" cy="15.5" r="2.5"></circle>
          <circle id="椭圆形备份-18" cx="2.5" cy="1.5" r="1.5"></circle>
          <circle id="椭圆形备份-23" cx="20.5" cy="1.5" r="1.5"></circle>
          <circle id="椭圆形备份-20" cx="2.5" cy="13.5" r="1.5"></circle>
          <circle id="椭圆形备份-29" cx="11.5" cy="21.5" r="1.5"></circle>
          <circle id="椭圆形备份-26" cx="11.5" cy="3.5" r="1.5"></circle>
          <circle id="椭圆形备份-27" cx="11.5" cy="9.5" r="1.5"></circle>
          <circle id="椭圆形备份-24" cx="20.5" cy="13.5" r="1.5"></circle>
          <circle id="椭圆形备份-30" cx="20.5" cy="18.5" r="1.5"></circle>
        </g>
      </g>
    </g>
  </svg>
)
const BHWandbIcon = (props) => <Icon component={BHWandb} {...props} />

const warenhouse = () => (
  <svg width="24px" height="24px" viewBox="0 0 24 24" version="1.1" >
    <g id="页面-1备份" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
      <g id="模型仓库" transform="translate(-12.000000, -339.000000)">
        <g id="编组-3" transform="translate(12.000000, 339.000000)">
          <g id="编组-2">
            <path d="M0.317987595,7.06448202 L11.7412651,0.156465151 C11.9003423,0.0602661555 12.0996577,0.0602661555 12.2587349,0.156465151 L23.6820124,7.06448202 C23.8793944,7.18384518 24,7.39772265 24,7.62838958 C24,7.83362435 23.8336243,8 23.6283896,8 L0.371610418,8 C0.166375651,8 2.513401e-17,7.83362435 0,7.62838958 C-2.82485524e-17,7.39772265 0.120605551,7.18384518 0.317987595,7.06448202 Z" id="矩形备份-74" fill="currentColor"></path>
            <path d="M1,7.70760212 L3,7 L3,23.5 C3,23.7761424 2.77614237,24 2.5,24 L1.5,24 C1.22385763,24 1,23.7761424 1,23.5 L1,7.70760212 L1,7.70760212 Z" id="矩形备份-63" fill="currentColor"></path>
            <path d="M21,7 L23,7.66504232 L23,23.5 C23,23.7761424 22.7761424,24 22.5,24 L21.5,24 C21.2238576,24 21,23.7761424 21,23.5 L21,7 L21,7 Z" id="矩形备份-75" fill="currentColor"></path>
            <rect id="矩形备份-69" fill="currentColor" x="5" y="18" width="6" height="6" rx="1"></rect>
            <rect id="矩形备份-76" fill="currentColor" x="13" y="18" width="6" height="6" rx="1"></rect>
            <rect id="矩形备份-77" fill="currentColor" x="9" y="10" width="6" height="6" rx="1"></rect>
            <rect id="矩形备份-78" fill="#FFFFFF" x="8" y="5" width="8" height="1" rx="0.5"></rect>
            <path d="" id="路径-19" stroke="currentColor"></path>
            <path d="" id="路径-19备份-3" stroke="currentColor" transform="translate(20.789486, 8.207602) scale(-1, 1) translate(-20.789486, -8.207602) "></path>
          </g>
          <rect id="矩形" fillOpacity="0" fill="#FFFFFF" x="2.27373675e-13" y="0" width="24" height="24"></rect>
        </g>
      </g>
    </g>
  </svg>
)

const BHWwarenhouse = (props) => <Icon component={warenhouse} {...props} />

const Icons = {
  BHEditIcon,
  BHSaveIcon,
  BHStopIcon,
  BHStartIcon,
  BHShareIcon,
  BHRestartIcon,
  BHDeleteIcon,
  //BHFormulaIcon,
  BHPackageIcon,
  BHStartAllIcon,
  BHStopAllIcon,
  BHAddFileIcon,
  BHAddFolderIcon,
  BHSearchIcon,
  BHTitleIcon,
  BHExtendAllIcon,
  BHUploadIcon,
  BHRefreshIcon,
  BHDatasetIcon,
  BHFolderIcon,
  BHCleanIcon,
  BHArrowDownIcon,
  BHArrowUpIcon,
  BHCellReadyIcon,
  BHCellPendingIcon,
  BHCellExecutingIcon,
  BHContactUsIcon,
  BHUploadFolderIcon,
  BHUploadFileIcon,
  BHTerminalIcon,
  BHDAGIcon,
  BHPipeLineIcon,
  BHPipeLineSelectedIcon,
  FileTreeShow,
  BHToBreakOf,
  BHToLink,
  BTOpenNewPage,
  BHStorageServiceIcon,
  BHDisconnectDatabase,
  BHRunPreCellsIcon,
  BHRunNextCellsIcon,
  BHCellResumeIcon,
  BHResumeRunIcon,
  BHTFBoardIcon,
  BHmonitorIcon,
  BHWandbIcon,
  BHWwarenhouse
}
export default Icons
