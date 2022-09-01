/**
 * 注册接口基类
 */
export default interface IRegister<T> {
    /**
     * 注册
     * @param name 名称
     * @param data 数据集
     */
    register(name: string, data: T, menuType?: string): void
    /**
     * 销毁
     * @param name 名称
     */
    destroyRegister(name: string): void
}