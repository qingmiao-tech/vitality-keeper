import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const projectRoot = path.resolve(scriptDir, '..')
const packageJsonPath = path.join(projectRoot, 'package.json')
const changelogPath = path.join(projectRoot, 'CHANGELOG.md')
const bundleRoot = path.join(projectRoot, 'src-tauri', 'target', 'release', 'bundle')
const outputDir = path.join(bundleRoot, 'updater')
const outputPath = path.join(outputDir, 'latest.json')

function normalizeRepositoryUrl(repository) {
  const value = typeof repository === 'string' ? repository : repository?.url
  if (!value) {
    throw new Error('package.json 中缺少 repository.url，无法生成 GitHub Release 下载地址。')
  }

  return value
    .replace(/^git\+/, '')
    .replace(/^git@github\.com:/, 'https://github.com/')
    .replace(/\.git$/, '')
}

function escapeRegex(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

function readReleaseNotes(version) {
  if (!fs.existsSync(changelogPath)) {
    return ''
  }

  const changelog = fs.readFileSync(changelogPath, 'utf8')
  const sections = changelog.split(/^##\s+/m).map((section) => section.trim()).filter(Boolean)
  const section = sections.find((entry) => {
    const heading = entry.split(/\r?\n/, 1)[0] || ''
    return heading.startsWith(`v${version}`) || heading.startsWith(version)
  }) || ''

  return section
    .split(/\r?\n/)
    .slice(1)
    .map((line) => line.trim())
    .filter(Boolean)
    .join('\n')
}

function pickWindowsArtifact(version) {
  const nsisDir = path.join(bundleRoot, 'nsis')
  const msiDir = path.join(bundleRoot, 'msi')
  const candidates = [
    path.join(nsisDir, `vitality-keeper_${version}_x64-setup.exe`),
    path.join(msiDir, `vitality-keeper_${version}_x64_en-US.msi`)
  ]

  const artifactPath = candidates.find((candidate) => fs.existsSync(candidate))
  if (!artifactPath) {
    throw new Error('未找到可用的 Windows 安装包，请先执行 npm run build。')
  }

  const signaturePath = `${artifactPath}.sig`
  if (!fs.existsSync(signaturePath)) {
    throw new Error(`未找到签名文件：${signaturePath}`)
  }

  return {
    artifactPath,
    artifactName: path.basename(artifactPath),
    signature: fs.readFileSync(signaturePath, 'utf8').trim()
  }
}

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'))
const version = String(packageJson.version || '').trim()
if (!version) {
  throw new Error('package.json 中缺少 version。')
}

const repositoryUrl = normalizeRepositoryUrl(packageJson.repository)
const releaseTag = process.env.VITALITY_KEEPER_RELEASE_TAG || `v${version}`
const releaseNotes = readReleaseNotes(version)
const windowsArtifact = pickWindowsArtifact(version)
const releaseAssetUrl = `${repositoryUrl}/releases/download/${releaseTag}/${windowsArtifact.artifactName}`

const manifest = {
  version: releaseTag,
  notes: releaseNotes,
  pub_date: new Date().toISOString(),
  platforms: {
    'windows-x86_64': {
      signature: windowsArtifact.signature,
      url: releaseAssetUrl
    }
  }
}

fs.mkdirSync(outputDir, { recursive: true })
fs.writeFileSync(outputPath, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8')

console.log(`Generated ${outputPath}`)
