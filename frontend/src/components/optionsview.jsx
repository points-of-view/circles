import Option from "./option";

export default function OptionsView({ options, tagsMap }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {options.map((key, index) => (
        <Option
          key={key}
          className="options-view__option"
          label={key}
          amount={tagsMap[index + 1]}
        />
      ))}
    </div>
  );
}
