type Props = {
  name: string
}

const FileLink: React.FC<Props> = ({ name }) => {
  return <button className="font-medium text-sm text-gray-700">{name}</button>
}

export default FileLink
