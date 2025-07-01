import http, { type BizResp } from './index.ts'

export interface TranslationTerm {
  unique_name: string,
  prop: string
  content: string
}

export async function get_all_translations(): Promise<BizResp<TranslationTerm[]>> {
  const { data } = await http.get<BizResp<TranslationTerm[]>>('/translations/all')
  return data
}


export async function save_translation(translation: TranslationTerm) {
  await http.post('/translations', translation, {
    headers: {
      'Content-Type': 'application/json'
    }
  })
}
