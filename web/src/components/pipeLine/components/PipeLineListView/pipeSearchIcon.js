import Icon from "@ant-design/icons"

const Init = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title>取消</title>
    <defs>
      <filter color-interpolation-filters="auto" id="filter-1">
        <feColorMatrix
          in="SourceGraphic"
          type="matrix"
          values="0 0 0 0 0.400000 0 0 0 0 0.400000 0 0 0 0 0.400000 0 0 0 1.000000 0"
        ></feColorMatrix>
      </filter>
      <filter color-interpolation-filters="auto" id="filter-2">
        <feColorMatrix
          in="SourceGraphic"
          type="matrix"
          values="0 0 0 0 0.400000 0 0 0 0 0.400000 0 0 0 0 0.400000 0 0 0 1.000000 0"
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
      <g id="任务备份" transform="translate(-700.000000, -597.000000)">
        <g id="取消" transform="translate(700.000000, 597.000000)">
          <rect
            id="矩形备份-13"
            fill="#946262"
            opacity="0"
            x="0"
            y="0"
            width="14"
            height="14"
          ></rect>
          <g filter="url(#filter-1)" id="编组-3备份-6">
            <g transform="translate(1.000000, 1.000000)">
              <circle
                id="椭圆形备份-25"
                stroke="#8C58EB"
                cx="6"
                cy="6"
                r="5.5"
              ></circle>
            </g>
          </g>
          <g filter="url(#filter-2)" id="编组-4">
            <g transform="translate(4.500000, 4.500000)">
              <line
                x1="0"
                y1="0"
                x2="5"
                y2="5"
                id="路径-28"
                stroke="#FF4D4F"
              ></line>
              <line
                x1="5"
                y1="0"
                x2="0"
                y2="5"
                id="路径-28备份"
                stroke="#FF4D4F"
              ></line>
            </g>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const InitIcon = (props) => <Icon component={Init} {...props} />

const Pending = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title>等待中</title>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="任务备份" transform="translate(-700.000000, -547.000000)">
        <g id="等待中" transform="translate(700.000000, 547.000000)">
          <rect
            id="矩形备份-10"
            fill="#946262"
            opacity="0"
            x="0"
            y="0"
            width="14"
            height="14"
          ></rect>
          <g
            id="编组-3备份-5"
            transform="translate(1.000000, 1.000000)"
            stroke="#1890FF"
          >
            <circle id="椭圆形备份" cx="6" cy="6" r="5.5"></circle>
            <line x1="3" y1="6" x2="9" y2="6" id="路径-21"></line>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const PendingIcon = (props) => <Icon component={Pending} {...props} />

const Schedule = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title>停止</title>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="任务备份" transform="translate(-700.000000, -621.000000)">
        <g id="停止" transform="translate(700.000000, 621.000000)">
          <rect
            id="矩形备份-14"
            fill="#946262"
            opacity="0"
            x="0"
            y="0"
            width="14"
            height="14"
          ></rect>
          <g
            id="编组-3备份-2"
            transform="translate(1.000000, 1.000000)"
            stroke="#8C58EB"
          >
            <circle id="椭圆形备份-25" cx="6" cy="6" r="5.5"></circle>
          </g>
          <g
            id="编组-6"
            transform="translate(5.000000, 4.500000)"
            stroke="#8C58EB"
          >
            <line x1="0.5" y1="0" x2="0.5" y2="5" id="路径-21备份"></line>
            <line x1="3.5" y1="0" x2="3.5" y2="5" id="路径-21备份-2"></line>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const ScheduleIcon = (props) => <Icon component={Schedule} {...props} />


const Success = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title>成功</title>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="任务备份" transform="translate(-700.000000, -572.000000)">
        <g id="成功" transform="translate(700.000000, 572.000000)">
          <rect
            id="矩形备份-11"
            fill="#946262"
            opacity="0"
            x="0"
            y="0"
            width="14"
            height="14"
          ></rect>
          <g
            id="编组-3"
            transform="translate(1.000000, 1.000000)"
            stroke="#00C400"
          >
            <circle id="椭圆形备份-25" cx="6" cy="6" r="5.5"></circle>
            <polyline
              id="路径-26备份"
              points="3.13425341 6.4711404 5.80226306 8.4711404 6.6554343 7.60160603 9.72698999 4.4711404"
            ></polyline>
          </g>
        </g>
      </g>
    </g>
  </svg>
)
const SuccessIcon = (props) => <Icon component={Success} {...props} />

const Running = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title>运行中</title>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="任务备份" transform="translate(-700.000000, -520.000000)">
        <g id="运行中" transform="translate(700.000000, 520.000000)">
          <rect
            id="矩形"
            fill="#FFFFFF"
            opacity="0"
            x="0"
            y="0"
            width="14"
            height="14"
          ></rect>
          <g
            id="编组-2"
            transform="translate(1.000000, 1.000000)"
            stroke="#FAAD14"
          >
            <g id="编组-3备份-4">
              <circle id="椭圆形备份" cx="6" cy="6" r="5.5"></circle>
              <line x1="3" y1="6" x2="8.5" y2="6" id="路径-21"></line>
            </g>
            <polyline
              id="路径-27"
              transform="translate(6.983451, 6.015024) rotate(-135.000000) translate(-6.983451, -6.015024) "
              points="5.96690285 5.04038091 5.96690285 6.98966714 8 6.98966714"
            ></polyline>
          </g>
        </g>
      </g>
    </g>
  </svg>
)

const RunningIcon = (props) => <Icon component={Running} {...props} />

const Fail = () => (
  <svg width="14px" height="14px" viewBox="0 0 14 14" version="1.1">
    <title>失败</title>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="任务备份" transform="translate(-700.000000, -648.000000)">
        <g id="失败" transform="translate(700.000000, 648.000000)">
          <rect
            id="矩形备份-17"
            fill="#946262"
            opacity="0"
            x="0"
            y="0"
            width="14"
            height="14"
          ></rect>
          <g id="编组-7" transform="translate(1.000000, 1.000000)">
            <g id="编组-3备份" stroke="#FF4D4F">
              <circle id="椭圆形备份-25" cx="6" cy="6" r="5.5"></circle>
            </g>
            <polygon
              id="矩形"
              fill="#FF4D4F"
              points="5.25 3 6.75 3 6.49230181 7 5.53507669 7"
            ></polygon>
            <circle id="椭圆形" fill="#FF4D4F" cx="6" cy="8.25" r="1"></circle>
          </g>
        </g>
      </g>
    </g>
  </svg>
)

const FailIcon = (props) => <Icon component={Fail} {...props} />
const InsideThePlan = () => (
  <svg width="12px" height="12px" viewBox="0 0 10 10" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
        <g id="工作流" transform="translate(-478.000000, -218.000000)" stroke="#FAAD14">
            <g id="编组-8备份-5" transform="translate(478.000000, 218.000000)">
                <circle id="椭圆形备份-2" cx="5" cy="5" r="4.5"></circle>
                <line x1="2.41149211" y1="5" x2="6.30533977" y2="5" id="路径-21备份"></line>
                <polyline id="路径-22" transform="translate(5.627198, 5.000175) rotate(-135.000000) translate(-5.627198, -5.000175) " points="4.68940632 4.06102588 4.68940632 5.93932494 6.56498873 5.93660829"></polyline>
            </g>
        </g>
    </g>
  </svg>
)
const InsideThePlanIcon = (props) => <Icon component={InsideThePlan} {...props} />


const NotPlanned = () => (
  <svg width="12px" height="12px" viewBox="0 0 10 10" version="1.1">
    <g id="页面-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
        <g id="工作流" transform="translate(-478.000000, -265.000000)" stroke="#FF4D4F">
            <g id="编组-2" transform="translate(478.000000, 265.000000)">
                <circle id="椭圆形备份-20" cx="5" cy="5" r="4.5"></circle>
                <polyline id="路径-25备份-2" points="2 6 4.19161386 4 6.00083894 6 8 4"></polyline>
            </g>
        </g>
    </g>
  </svg>
)
const NotPlannedIcon = (props) => <Icon component={NotPlanned} {...props} />

const PipeSearchIcon = {
  Init,
  InitIcon,
  Schedule,
  ScheduleIcon,
  NotPlannedIcon,
  InsideThePlanIcon,
  Pending,
  PendingIcon,
  Success,
  SuccessIcon,
  Running,
  RunningIcon,
  Fail,
  FailIcon,
}

export default PipeSearchIcon
