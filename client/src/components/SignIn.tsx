type SignInProps = {
  message: string
  hasSavedToken: boolean
  worldName: string
  moduleName: string
  serverUri: string
  onSignIn: () => void
  onForgetSavedToken: () => void
}

export function SignIn({
  message,
  hasSavedToken,
  worldName,
  moduleName,
  serverUri,
  onSignIn,
  onForgetSavedToken,
}: SignInProps) {
  return (
    <>
      <h1 className="title">Ikaria</h1>
      <p className="subtitle">Sign in to Alpha world</p>

      <div className="metadata-row">
        <span>World</span>
        <strong>{worldName}</strong>
      </div>
      <div className="metadata-row">
        <span>Module</span>
        <code>{moduleName}</code>
      </div>
      <div className="metadata-row">
        <span>Server</span>
        <code>{serverUri}</code>
      </div>
      <div className="metadata-row">
        <span>Saved token</span>
        <strong>{hasSavedToken ? 'Available' : 'None'}</strong>
      </div>

      <p className="status status-idle">{message}</p>

      <div className="actions">
        <button type="button" onClick={onSignIn}>
          Sign in to Alpha world
        </button>
        <button
          type="button"
          className="button-secondary"
          onClick={onForgetSavedToken}
        >
          Forget saved token
        </button>
      </div>
    </>
  )
}
