$('.formInput input, .formInput select').each(function() {
	if ($(this).val() !== '') {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	}
	$(this).focusin(() => {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	});
	$(this).focusout(function() {
		if ($(this).val() === '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '1rem',
				'top': '1rem',
				'padding': 0
			}, 80);
		}
	});
	$(this).keyup(function () {
		$(this).parent().attr('class', 'formInput');
		$(`#${this.id} ~ .helper-text`).empty();
	});
});

$('form').submit(e => {
	e.preventDefault();
	if ($('#password').val().length === 0) {
		$('#password').parent().attr('class', 'formInput error');
		$('#password ~ .helper-text').text('Enter the password');
		showToast('Enter the password');
		return;
	}
	showToast('Please wait...', 10000);
	$.ajax({
		type: 'POST',
		url: '/admin_signin',
		data: {
			scope: $('#scope').val(),
			password: $('#password').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.reload();
		}
	});
});

$('#submissions .cover').each(function() {
	$(this).click(function() {
		$('.cover').show();
		$('.detailedView').hide();
		$(this.parentNode.childNodes[5]).show();
		$(this).hide();
	});
});

$('#submissions .detailedView').each(function() {
	$(this).click(function() {
		$('.cover').show();
		$('.detailedView').hide();
	});
});

$('#signoutButton').click(() => {
	$.ajax({
		type: 'POST',
		url: '/admin_signout',
		success: (result) => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.reload();
		}
	});
});
