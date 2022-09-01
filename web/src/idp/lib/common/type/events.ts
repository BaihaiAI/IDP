export default interface Events {
    key?: string;
    item?: React.ReactInstance;
    domEvent?: React.MouseEvent<HTMLElement> | React.KeyboardEvent<HTMLElement>;
}