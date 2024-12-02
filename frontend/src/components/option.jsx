import clsx from "clsx";
import Token from "./token";

export default function Option({
  className = "",
  label,
  amount = 0,
  big = false,
  tagIds = [],
}) {
  return (
    <div className={clsx("option", className, { "option--big": big })}>
      {!big && <div className="option__amount">{amount}</div>}
      <div className="option__content">
        <div className="option__label">{label}</div>
        {!big && (
          <div className="option__figure-container">
            {tagIds.map((value) => (
              <Token tagID={value} key={value} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
