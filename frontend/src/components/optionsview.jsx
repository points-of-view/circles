import Option from "./option";

export default function OptionsView() {
  return (
    <div className="optionsview">
      <Option label={"Optie 1"} amount={25} />
      <Option label={"Optie 2"} amount={13} />
      <Option label={"Optie 3"} amount={0} />
    </div>
  );
}
