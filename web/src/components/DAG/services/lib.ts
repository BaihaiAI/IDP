import dayjs from 'dayjs'

export const formatDateTime = (dateTime) => {
  return dateTime ? dayjs(new Date(dateTime)).format("YYYY-MM-DD HH:mm:ss") : ''
}

export const formatCostTime = (time: number) => {
  const duration = Number(time)
  if (isNaN(duration)) return time
  let executionTime = ""
  if (duration < 60) {
    executionTime = `${duration} s`
  } else if (duration >= 60 && duration < 3600) {
    const min = Math.floor(duration / 60)
    const s = Math.floor(duration % 60)
    executionTime = `${min} min ${s} s`
  } else if (duration >= 3600) {
    const hour = Math.floor(duration / 3600)
    const min = Math.floor((duration % 3600) / 60)
    executionTime = `${hour} hour ${min} min`
  }
  return executionTime
}