!(function () {
	/**
	 * Write contents to output area.
	 * @param {string} contents
	 */
	var writeContents = function (contents) {
		$('#contents').text(contents)
	}

	/**
	 * Write error with payload.
	 * @param {string} message
	 */
	var writeError = function (message) {
		writeContents('Error!\n\n' + message)
	}

	$.ajax({
		url: 'http://jquery.com/',
		type: 'GET',
		success: function (res) {
			if (!res || !res.responseText) {
				writeError('Success callback recieved invalid response:\n-----\n' + JSON.stringify(res))
				return
			}

			writeContents(res.responseText)
		},
		error: function (xhr, status, error) {
			writeError('An error has occured:\n' + status + '\n' + error.message)
		}
	})
})()
