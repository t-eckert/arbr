import { useDoc } from "../hooks"

const Doc = () => {
  const { data } = useDoc("/ReadingList.md")

  return (
    <section className="px-4 py-2 flex-1">
      <article className="mx-auto w-full max-w-xl">
        <div
          className="markdown"
          dangerouslySetInnerHTML={{ __html: data || "" }}
        />
      </article>
    </section>
  )
}

export default Doc
