import { useQuery, UseQueryResult } from "react-query";

const useDoc = (dir: string): UseQueryResult<string> => {
	return useQuery(`dir-${dir}`, async (): Promise<string> => {
		const response = await fetch(`http://localhost:8000/api/files/html${dir}`)
		if (!response.ok) {
			throw new Error(response.statusText)
		}

		const data = await response.text()
		return data
	})
}

export default useDoc