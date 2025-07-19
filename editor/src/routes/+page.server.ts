
import type { ImageDescriptors } from "$lib/types/imageDescriptor";
import type { PageServerLoad } from "./$types";


export const load: PageServerLoad = async ({ fetch }) => {
  let response = await fetch('http://localhost:3000/editor/image/all');

  if (response.status == 200) {
    let body = await response.json() as ImageDescriptors[]

console.log(body);

    return {
      images: body
    }
  }
}
