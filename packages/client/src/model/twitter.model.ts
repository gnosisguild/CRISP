export interface Twitter {
  open_graph: OpenGraph
  theme_color: string
  description: string
  favicon: string
}

export interface OpenGraph {
  site_name: string
  type: string
  url: string
  title: string
  description: string
  images: Image[]
}

export interface Image {
  url: string
}

export interface SocialAuth {
  validationDate: Date
  avatar: string
  username: string
  postId: string
  token: string
}

export interface Auth {
  jwt_token: string
  response: 'Already Authorized' | 'No Authorization'
}
