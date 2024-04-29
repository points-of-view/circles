import Option from "./option";

export default function OptionsView({ options, tagCount }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {options.list.map((key, index) => (
        <Option
          key={key}
          className="options-view__option"
          label={key}
          showDescriptionLayout={options.showDescriptionLayout}
          big={options.list.length === 1}
          amount={tagCount[index + 1]}
        />
      ))}
    </div>
  );
}
