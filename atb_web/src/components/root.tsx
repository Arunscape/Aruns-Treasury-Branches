import { Outlet } from "react-router-dom"

export default () => {
    return <>
    <nav>
        <div>this is the navbar</div>
    </nav>
    <Outlet/>
    </>
}