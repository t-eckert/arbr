import { useQuery } from "react-query"

import { DirLink, FileLink } from "../components"

const Nav = () => {
  const { data, status } = useQuery("nav", () =>
    fetch("http://localhost:8000/api/dir").then((res) => res.json())
  )

  return (
    <nav className="py-2 px-4 w-60 flex flex-col items-start gap-1 bg-gray-50">
      {data.directories.map((directory: string) => (
        <DirLink key={`dir-${directory}`} name={directory} />
      ))}
      {data.markdown.map((file: string) => (
        <FileLink key={`file-${file}`} name={file} />
      ))}
    </nav>
  )
}

export default Nav
