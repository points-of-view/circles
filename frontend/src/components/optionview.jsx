import Option from "./option";

export default function OptionView() {
    return (
        <div className="optionview">
            <Option label={"Optie 1"} amount={25} />
            <Option label={"Optie 2"} amount={13} />
            <Option label={"Optie 3"} amount={0} />
        </div>
    )
}