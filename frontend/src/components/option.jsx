export default function Option({ label, tags }) {
    return (
        <div className="option">
            {tags != 0 && <div className="option__amount">
                {tags}
            </div>}
            <div className="option__label">
                {label}
            </div>
            <div className="option__silhouettes">
                {tags
                  .for((index, label) => (
                    <Shape
                      key={label}
                      shape={"figure"}
                      className="people-figure"
                    />
                  ))}
              </div>
        </div>
    )
}