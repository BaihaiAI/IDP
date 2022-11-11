/// <reference types="react" />
declare interface Props {
    children?: string;
    linkify?: boolean;
    className?: string;
    useClasses?: boolean;
    clickHandle?: FunctionStringCallback | any;
    errorConfig?: any;
}
export default function Ansi(props: Props): JSX.Element;
export {};
