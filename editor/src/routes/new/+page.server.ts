import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async ({request,  fetch}) => {
    let formData = await request.formData();

    console.log(formData);

    let response = await fetch('http://localhost:3000/editor/image/new', { body: formData, method: "POST" });

    console.log(`${response.status}: ${await response.text()}`)

    if (response.status == 201) {
      redirect(302, "/");
    }
  }
} satisfies Actions;
