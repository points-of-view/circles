import Option from "./option";

export default function OptionsView({ options }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {options.map((value, index) => (
        <Option
          key={index}
          className="options-view__option"
          label={value}
          amount={0}
        />
      ))}
    </div>
  );
}
