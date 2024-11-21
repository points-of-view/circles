import Option from "./option";
import clsx from "clsx";

export default function OptionsView({ options, tagCount, tagsMap }) {
  const tagIds = Object.values(tagsMap).reduce((tag, { id, antenna }) => {
    if (!tag[antenna]) {
      tag[antenna] = [];
    }
    tag[antenna].push(id);
    return tag;
  }, {});
  return (
    <div className="options-view interaction-screen__option-view">
      {options.map((option, index) => (
        <Option
          key={option.value}
          className={clsx(
            "options-view__option",
            option.color && `options-view__option--${option.color}`,
          )}
          label={option.value}
          big={options.length === 1}
          amount={tagCount[index + 1]}
          tagIds={tagIds[index + 1]}
        />
      ))}
    </div>
  );
}
