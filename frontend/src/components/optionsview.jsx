import Option from "./option";

export default function OptionsView({ options, tagCount }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {options.list.map((option, index) => (
        <Option
          key={option.value}
          className={
            "options-view__option" + options.color &&
            "options-view__option--" + option.color
          }
          label={option.value}
          showDescriptionLayout={options.showDescriptionLayout}
          big={options.list.length === 1}
          amount={tagCount[index + 1]}
        />
      ))}
    </div>
  );
}
