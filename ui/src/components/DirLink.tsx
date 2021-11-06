type Props = {
  name: string
}

const DirLink: React.FC<Props> = ({ name }) => {
  return (
    <button className="font-medium text-sm text-gray-700">{name}&nbsp;/</button>
  )
}

export default DirLink
