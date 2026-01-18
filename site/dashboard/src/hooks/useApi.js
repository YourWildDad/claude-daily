import { useState, useCallback } from 'react'

const API_BASE = '/api'

export function useApi() {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)

  const request = useCallback(async (endpoint, options = {}) => {
    setLoading(true)
    setError(null)
    try {
      const res = await fetch(`${API_BASE}${endpoint}`, {
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
        ...options,
      })
      const data = await res.json()
      if (!data.success) {
        throw new Error(data.error || 'Request failed')
      }
      return data.data
    } catch (err) {
      setError(err.message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [])

  const fetchDates = useCallback(() => request('/dates'), [request])

  const fetchDailySummary = useCallback(
    (date) => request(`/dates/${date}`),
    [request]
  )

  const fetchSessions = useCallback(
    (date) => request(`/dates/${date}/sessions`),
    [request]
  )

  const fetchSession = useCallback(
    (date, name) => request(`/dates/${date}/sessions/${encodeURIComponent(name)}`),
    [request]
  )

  const fetchJobs = useCallback(() => request('/jobs'), [request])

  const fetchJob = useCallback(
    (id) => request(`/jobs/${id}`),
    [request]
  )

  const fetchJobLog = useCallback(
    (id) => request(`/jobs/${id}/log`),
    [request]
  )

  const killJob = useCallback(
    (id) => request(`/jobs/${id}/kill`, { method: 'POST' }),
    [request]
  )

  return {
    loading,
    error,
    fetchDates,
    fetchDailySummary,
    fetchSessions,
    fetchSession,
    fetchJobs,
    fetchJob,
    fetchJobLog,
    killJob,
  }
}
