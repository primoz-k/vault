import Foundation
import VaultMobile

public enum RemoteFilesDirPickerRoute: Hashable {
    case dirPicker(location: String)
}

public typealias RemoteFilesDirPickerNavController = NavController<RemoteFilesDirPickerRoute>
