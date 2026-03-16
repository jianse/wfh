import http, { type BizResp, type CustomRequestConfig } from './index.ts'

export interface TranslationTerm {
  unique_name: string
  prop: string
  content: string
}

export interface TranslationId {
  unique_name: string
  prop: string
}

export async function get_all_translations(): Promise<BizResp<TranslationTerm[]>> {
  const { data } = await http.get<BizResp<TranslationTerm[]>>('/translations/all', {
    skipAuth: true,
  } as CustomRequestConfig)
  return data
}

export async function save_translation(translation: TranslationTerm) {
  await http.post('/translations', translation)
}

export async function delete_translation(translation_id: TranslationId) {
  await http.delete('/translations', {
    params: translation_id,
  })
}
