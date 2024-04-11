import Option from "./option";

export default function OptionsView() {
  return (
    <div className="options-view interaction-screen__option-view">
      <Option className="options-view__option" label={"Optie 1"} amount={25} />
      <Option className="options-view__option" label={"Optie 2"} amount={13} />
      <Option className="options-view__option" label={"Optie 3"} amount={0} />
    </div>
  );
}
