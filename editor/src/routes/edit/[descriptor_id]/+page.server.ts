import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";


import type { ImageDescriptors } from "$lib/types/imageDescriptor";
import type { PageServerLoad } from "./$types";


export const load: PageServerLoad = async ({ fetch, params }) => {
  let response = await fetch(`http://localhost:3000/editor/image/${params.descriptor_id}`);

  if (response.status == 200) {
    let body = await response.json() as ImageDescriptors

    return {
      image: body
    }
  }
}

export const actions = {
  default: async ({ request, fetch, params }) => {
    let formData = await request.formData();

    console.log(formData);

    let response = await fetch(`http://localhost:3000/editor/image/${params.descriptor_id}/update`, { body: formData, method: "POST" });

    console.log(`${response.status}: ${await response.text()}`)

    if (response.status == 200) {
      redirect(302, "/");
    }
  }
} satisfies Actions;
