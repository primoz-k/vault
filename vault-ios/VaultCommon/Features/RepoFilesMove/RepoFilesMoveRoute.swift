import Foundation
import VaultMobile

public enum RepoFilesMoveRoute: Hashable {
    case repoFiles(repoId: String, path: String)
}

public typealias RepoFilesMoveNavController = NavController<RepoFilesMoveRoute>
