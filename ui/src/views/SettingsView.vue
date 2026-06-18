<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useToast } from 'primevue/usetoast'
import { get_current_user } from '@/api/user'
import { uint8ArrayToBase64, hexToUint8Array } from '@/util/common'

const toast = useToast()
const username = ref('')
const role = ref('')

// Passkey related state
const passkeys = ref<Array<{ id: string; name: string; created_at: string }>>([])
const newPasskeyName = ref('test')
const registering = ref(false)

onMounted(async () => {
  try {
    const { data } = await get_current_user()
    if (data) {
      username.value = data.username
      role.value = data.role
    }
    // TODO: Load user's passkeys when backend API is ready
    // loadPasskeys()
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to load user information',
      life: 3000,
    })
  }
})

async function registerPasskey() {
  if (!newPasskeyName.value.trim()) {
    toast.add({
      severity: 'warn',
      summary: 'Warning',
      detail: 'Please enter a name for your passkey',
      life: 3000,
    })
    return
  }

  registering.value = true
  try {
    // TODO: Implement WebAuthn registration
    // This is a placeholder for the actual WebAuthn implementation

    // Step 1: Get challenge from server
    // const response = await http.post('/user/passkey/register/start', {
    //   name: newPasskeyName.value
    // })
    // const options = response.data

    // Step 2: Create credential using WebAuthn API
    const credential = await navigator.credentials.create({
      publicKey: {
        challenge: hexToUint8Array('1234567890abcdef1234567890abcdef'),
        rp: { name: 'Example RP', id: 'localhost' },
        user: { id: hexToUint8Array('00'), name: 'username', displayName: 'Username' },
        pubKeyCredParams: [{ type: 'public-key', alg: -7 }],
      },
    })
    if (credential) {
      const c = credential as PublicKeyCredential
      console.log(c)
      const resp = c.response as AuthenticatorAttestationResponse
      const decoder = new TextDecoder('utf-8')
      // const base64
      // const text = decoder.decode(buffer);
      console.log(uint8ArrayToBase64(new Uint8Array(resp.attestationObject)))
      // resp.getAuthenticatorData()
      console.log(decoder.decode(resp.clientDataJSON))
    }

    // console.log(credential.response.attestationObject)

    // Step 3: Send credential back to server
    // await http.post('/user/passkey/register/finish', {
    //   credential: credential,
    //   name: newPasskeyName.value
    // })

    // Simulated success for now
    await new Promise((resolve) => setTimeout(resolve, 1000))

    toast.add({
      severity: 'success',
      summary: 'Success',
      detail: 'Passkey registered successfully',
      life: 3000,
    })

    newPasskeyName.value = ''
    // Reload passkeys
    // await loadPasskeys()
  } catch (e) {
    console.error('Failed to register passkey', e)
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to register passkey',
      life: 3000,
    })
  } finally {
    registering.value = false
  }
}

async function deletePasskey() {
  try {
    // TODO: Implement passkey deletion
    // await http.delete(`/user/passkey/${passkeyId}`)

    toast.add({
      severity: 'success',
      summary: 'Success',
      detail: 'Passkey deleted successfully',
      life: 3000,
    })

    // Reload passkeys
    // await loadPasskeys()
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to delete passkey',
      life: 3000,
    })
  }
}

function isWebAuthnSupported() {
  return window.PublicKeyCredential !== undefined
}
</script>

<template>
  <div class="settings-container">
    <div class="card">
      <h2>Account Settings</h2>

      <div class="field-group">
        <label>Username</label>
        <PvInputText v-model="username" disabled />
      </div>

      <div class="field-group">
        <label>Role</label>
        <PvInputText v-model="role" disabled />
      </div>
    </div>

    <div class="card">
      <h2>Passkey Authentication</h2>

      <div v-if="!isWebAuthnSupported()" class="warning-message">
        <i class="pi pi-exclamation-triangle"></i>
        <span>Your browser does not support WebAuthn/Passkeys</span>
      </div>

      <div v-else>
        <div class="passkey-registration">
          <h3>Register New Passkey</h3>
          <div class="registration-form">
            <pv-input-text
              v-model="newPasskeyName"
              placeholder="Enter a name for this passkey (e.g., 'My Phone', 'Work Laptop')"
              :disabled="registering"
            />
            <pv-button
              label="Register Passkey"
              icon="pi pi-key"
              @click="registerPasskey"
              :loading="registering"
              :disabled="!newPasskeyName.trim()"
            />
          </div>
          <small class="help-text">
            Passkeys allow you to sign in without a password using your device's biometric
            authentication or PIN.
          </small>
        </div>

        <div class="passkey-list" v-if="passkeys.length > 0">
          <h3>Your Passkeys</h3>
          <PvDataTable :value="passkeys" tableStyle="min-width: 50rem">
            <PvColumn field="name" header="Name"></PvColumn>
            <PvColumn field="created_at" header="Created"></PvColumn>
            <PvColumn header="Actions" style="width: 100px">
              <template #body>
                <PvButton
                  icon="pi pi-trash"
                  severity="danger"
                  text
                  rounded
                  @click="deletePasskey"
                />
              </template>
            </PvColumn>
          </PvDataTable>
        </div>

        <div v-else class="empty-state">
          <i class="pi pi-key" style="font-size: 3rem; color: var(--text-color-secondary)"></i>
          <p>No passkeys registered yet</p>
          <small>Add a passkey above to enable passwordless authentication</small>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-container {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.card {
  background: var(--surface-card);
  border-radius: 8px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

h2 {
  margin-top: 0;
  margin-bottom: 1.5rem;
  color: var(--text-color);
  font-size: 1.5rem;
}

h3 {
  margin-top: 0;
  margin-bottom: 1rem;
  color: var(--text-color);
  font-size: 1.2rem;
}

.field-group {
  margin-bottom: 1.5rem;
}

.field-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--text-color);
}

.field-group .p-inputtext {
  width: 100%;
  max-width: 400px;
}

.passkey-registration {
  margin-bottom: 2rem;
}

.registration-form {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}

.registration-form .p-inputtext {
  flex: 1;
  max-width: 400px;
}

.help-text {
  color: var(--text-color-secondary);
  font-size: 0.875rem;
  display: block;
  margin-top: 0.5rem;
}

.warning-message {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background: var(--orange-50);
  border: 1px solid var(--orange-200);
  border-radius: 6px;
  color: var(--orange-700);
  margin-bottom: 1.5rem;
}

.dark .warning-message {
  background: var(--orange-900);
  border-color: var(--orange-700);
  color: var(--orange-200);
}

.passkey-list {
  margin-top: 2rem;
}

.empty-state {
  text-align: center;
  padding: 3rem 1rem;
  color: var(--text-color-secondary);
}

.empty-state p {
  margin: 1rem 0 0.5rem;
  font-size: 1.1rem;
}

.empty-state small {
  color: var(--text-color-secondary);
}
</style>
