import Combine
import SwiftUI
import UniformTypeIdentifiers
import VaultMobile

public class RepoFilesScreenViewModel: ObservableObject, WithRepoGuardViewModel {
    public let container: Container
    public let navController: MainNavController
    public let repoId: String
    public let encryptedPath: String

    public let browserId: UInt32

    @Published var filesImporterIsPresented: Bool = false
    @Published var filesImporterAllowedContentTypes: [UTType] = []
    @Published var filesImporterAllowsMultipleSelection: Bool = false

    @Published var editMode: EditMode = .inactive
    @Published var selection = Set<String>()

    @Published public var info: VaultMobile.Subscription<RepoFilesBrowserInfo>

    @Published public var repoGuardViewModel: RepoGuardViewModel

    private var selectionChangedCancellable: AnyCancellable?

    private var isUpdatingSelection: Bool = false

    public init(
        container: Container, navController: MainNavController, repoId: String,
        encryptedPath: String
    ) {
        self.container = container
        self.navController = navController
        self.repoId = repoId
        self.encryptedPath = encryptedPath

        let browserId = container.mobileVault.repoFilesBrowsersCreate(
            repoId: repoId, encryptedPath: encryptedPath,
            options: RepoFilesBrowserOptions(selectName: nil))

        self.browserId = browserId

        info = VaultMobile.Subscription(
            mobileVault: container.mobileVault,
            subscribe: { v, cb in
                v.repoFilesBrowsersInfoSubscribe(browserId: browserId, cb: cb)
            },
            getData: { v, id in
                v.repoFilesBrowsersInfoData(id: id)
            })

        repoGuardViewModel = RepoGuardViewModel(
            container: container, repoId: repoId, setupBiometricUnlockVisible: true)

        selectionChangedCancellable = self.$selection.sink { [weak self] selection in
            if let self = self {
                if !isUpdatingSelection {
                    self.updateItems(from: selection)
                }
            }
        }

        info.setOnData { [weak self] data in
            if let self = self {
                if let info = data {
                    self.repoGuardViewModel.update(
                        repoStatus: info.repoStatus, isLocked: info.isLocked)

                    if !info.isLocked {
                        self.updateSelection(from: info.items)
                    }
                }
            }
        }
    }

    deinit {
        container.mobileVault.repoFilesBrowsersDestroy(browserId: browserId)
    }

    func startEditMode() {
        editMode = .active
    }

    func stopEditMode() {
        editMode = .inactive
    }

    func updateItems(from selection: Set<String>) {
        if let info = info.data {
            let selectedItems = Set(info.items.filter { $0.isSelected }.map { $0.file.id })

            if selection != selectedItems {
                container.mobileVault.repoFilesBrowsersSetSelection(
                    browserId: browserId, selection: [String](selection))
            }
        }
    }

    func updateSelection(from items: [RepoFilesBrowserItem]) {
        isUpdatingSelection = true

        if items.isEmpty {
            stopEditMode()
        } else {
            let selectedItems = Set(items.filter { $0.isSelected }.map { $0.file.id })

            for id in selectedItems {
                if !selection.contains(id) {
                    selection.insert(id)
                }
            }

            for id in selection {
                if !selectedItems.contains(id) {
                    selection.remove(id)
                }
            }
        }

        isUpdatingSelection = false
    }

    func uploadFiles() {
        filesImporterAllowedContentTypes = [UTType.data]
        filesImporterAllowsMultipleSelection = true
        filesImporterIsPresented = true
    }

    func uploadFolder() {
        filesImporterAllowedContentTypes = [UTType.folder]
        filesImporterAllowsMultipleSelection = false
        filesImporterIsPresented = true
    }
}
