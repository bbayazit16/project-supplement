import { DaoCard } from "./DaoCard"

const DaoGrid = props => {
    return (
        <div className="flex flex-wrap">
            {props.cards.map(cardProp => {
                return (
                    <div className="px-4">
                        <DaoCard {...cardProp} />
                    </div>
                )
            })}
        </div>
    )
}

export default DaoGrid
