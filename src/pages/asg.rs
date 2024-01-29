use leptos::*;

#[component]
pub fn CreateASG() -> impl IntoView {
    view! {
        <h1>"Create New ASG"</h1>

        <div>
            <label>"ASG Name" <input type="text" name="name"/></label>

            <label>"Instance Count" <input type="number" name="count"/></label>
        </div>

        <div>
            <label>"Capacity Rebalance" <input type="checkbox" name="capacity_rebalance"/></label>

            <label for="Instance Type">Select an Instance Type:</label>

            <select name="Instance Type" id="Instance Type">
                <option value="volvo">Volvo</option>
                <option value="saab">Saab</option>
                <option value="mercedes">Mercedes</option>
                <option value="audi">Audi</option>
            </select>
        </div>

        <div>
            <label for="Launch Template">Select a Launch Template</label>

            <select name="Launch Template" id="Launch Template">
                <option value="volvo">Volvo</option>
                <option value="saab">Saab</option>
                <option value="mercedes">Mercedes</option>
                <option value="audi">Audi</option>
            </select>

            <label for="Subnets">Select Subnets</label>

            <select name="Subnets" id="Subnets">
                <option value="volvo">Volvo</option>
                <option value="saab">Saab</option>
                <option value="mercedes">Mercedes</option>
                <option value="audi">Audi</option>
            </select>

        </div>

        <div>
            <input type="submit" value="Add"/>
        </div>
    }
}
