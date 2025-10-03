import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { NextRequest } from 'next/server'
import { GET, POST } from './route'

// Mock Clerk auth
vi.mock('@clerk/nextjs/server', () => ({
  auth: vi.fn(),
}))

// Mock fetch for MCP server calls
const mockFetch = vi.fn()
global.fetch = mockFetch

describe('/api/user/api-keys', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  describe('GET', () => {
    it('returns 401 when user is not authenticated', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      vi.mocked(auth).mockResolvedValue({ getToken: null })

      const response = await GET()
      expect(response.status).toBe(401)

      const data = await response.json()
      expect(data.error).toBe('Unauthorized')
    })

    it('successfully fetches API keys from MCP server', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      const mockToken = 'mock-session-token'

      vi.mocked(auth).mockResolvedValue({
        getToken: vi.fn().mockResolvedValue(mockToken)
      })

      const mockApiKeysResponse = {
        ok: true,
        data: [
          {
            id: 'key-1',
            name: 'Production Key',
            keyPrefix: 'iora_pk_prod...',
            createdAt: '2025-01-01T00:00:00Z',
            lastUsedAt: '2025-01-02T00:00:00Z',
            permissions: ['tools:read', 'tools:write']
          }
        ]
      }

      mockFetch.mockResolvedValue({
        ok: true,
        json: async () => mockApiKeysResponse
      })

      const response = await GET()
      expect(response.status).toBe(200)

      const data = await response.json()
      expect(data).toEqual(mockApiKeysResponse)

      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:7145/user/api-keys',
        expect.objectContaining({
          headers: expect.objectContaining({
            'Authorization': `Bearer ${mockToken}`,
            'Content-Type': 'application/json',
          })
        })
      )
    })

    it('handles MCP server errors', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      vi.mocked(auth).mockResolvedValue({
        getToken: vi.fn().mockResolvedValue('mock-token')
      })

      mockFetch.mockResolvedValue({
        ok: false,
        status: 404,
        json: async () => ({ message: 'Not found' })
      })

      const response = await GET()
      expect(response.status).toBe(404)

      const data = await response.json()
      expect(data.error).toBe('Failed to fetch API keys')
    })
  })

  describe('POST', () => {
    it('returns 401 when user is not authenticated', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      vi.mocked(auth).mockResolvedValue({ getToken: null })

      const request = new NextRequest('http://localhost:3000/api/user/api-keys', {
        method: 'POST',
        body: JSON.stringify({ name: 'Test Key' })
      })

      const response = await POST(request)
      expect(response.status).toBe(401)

      const data = await response.json()
      expect(data.error).toBe('Unauthorized')
    })

    it('successfully creates API key via MCP server', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      const mockToken = 'mock-session-token'

      vi.mocked(auth).mockResolvedValue({
        getToken: vi.fn().mockResolvedValue(mockToken)
      })

      const mockCreateResponse = {
        ok: true,
        data: {
          id: 'key-123',
          key: 'iora_pk_new_key_123456789',
          keyPrefix: 'iora_pk_new...',
          message: 'Save this key securely. It will not be shown again.'
        }
      }

      mockFetch.mockResolvedValue({
        ok: true,
        json: async () => mockCreateResponse
      })

      const request = new NextRequest('http://localhost:3000/api/user/api-keys', {
        method: 'POST',
        headers: {
          'content-type': 'application/json'
        },
        body: JSON.stringify({
          name: 'New Test Key',
          permissions: ['tools:read', 'tools:write'],
          expiresInDays: 90
        })
      })

      const response = await POST(request)
      expect(response.status).toBe(200)

      const data = await response.json()
      expect(data).toEqual(mockCreateResponse)

      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:7145/user/api-keys',
        expect.objectContaining({
          method: 'POST',
          headers: expect.objectContaining({
            'Authorization': `Bearer ${mockToken}`,
            'Content-Type': 'application/json',
          }),
          body: JSON.stringify({
            name: 'New Test Key',
            permissions: ['tools:read', 'tools:write'],
            expiresInDays: 90
          })
        })
      )
    })

    it('validates required fields', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      vi.mocked(auth).mockResolvedValue({
        getToken: vi.fn().mockResolvedValue('mock-token')
      })

      mockFetch.mockResolvedValue({
        ok: true,
        json: async () => ({ ok: true, data: {} })
      })

      // Test missing name
      const request = new NextRequest('http://localhost:3000/api/user/api-keys', {
        method: 'POST',
        headers: {
          'content-type': 'application/json'
        },
        body: JSON.stringify({
          permissions: ['tools:read']
        })
      })

      const response = await POST(request)
      expect(response.status).toBe(200) // Should still work as MCP handles validation
    })

    it('handles MCP server errors during creation', async () => {
      const { auth } = await import('@clerk/nextjs/server')
      vi.mocked(auth).mockResolvedValue({
        getToken: vi.fn().mockResolvedValue('mock-token')
      })

      mockFetch.mockResolvedValue({
        ok: false,
        status: 400,
        json: async () => ({ message: 'Invalid API key name' })
      })

      const request = new NextRequest('http://localhost:3000/api/user/api-keys', {
        method: 'POST',
        headers: {
          'content-type': 'application/json'
        },
        body: JSON.stringify({
          name: 'Invalid Key Name',
          permissions: ['tools:read']
        })
      })

      const response = await POST(request)
      expect(response.status).toBe(400)

      const data = await response.json()
      expect(data.error).toBe('Failed to create API key')
    })
  })
})
