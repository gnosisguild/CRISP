import { EMOJI_LIST } from '@/mocks/polls'
import { Poll } from '@/model/poll.model'

export const generateRandomPoll = (): Poll[] => {
  let index1 = Math.floor(Math.random() * EMOJI_LIST.length)
  let index2 = Math.floor(Math.random() * EMOJI_LIST.length)
  while (index1 === index2) {
    index2 = Math.floor(Math.random() * EMOJI_LIST.length)
  }

  return [
    {
      value: 0,
      label: EMOJI_LIST[index1],
      checked: false,
    },
    { value: 1, label: EMOJI_LIST[index2], checked: false },
  ]
}
