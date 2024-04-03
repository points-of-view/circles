import Shape from "./shape";

export default function Option({ label, amount = 0, option }) {
  return (
    <div
      className="option"
      style={{
        "--option-color": `var(--option-color-${option.key})`,
      }}
    >
      <div className="option__amount">{amount}</div>
      <div className="option__container">
        <div className="option__content" >
          <div className="option__label">{label}</div>
          {amount != 0 && (
            <div className="option__silhouettes">
              {[...Array(amount)].map((i, e) => (
                <Shape key={e} shape={"figure"} className="figure" />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
